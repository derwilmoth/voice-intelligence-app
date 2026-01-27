use crate::audio::play_sound;
use crate::logic::{set_status, AppStatus};
use crate::models::HistoryItem;
use crate::store::{load_data, save_data};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::fs::File;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

const OLLAMA_API_URL: &str = "http://localhost:11434/api/chat";
const MODEL_URL: &str =
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo-q8_0.bin";
const MODEL_FILENAME: &str = "ggml-large-v3-turbo-q8_0.bin";

pub fn run_pipeline(app: AppHandle) {
    std::thread::spawn(move || {
        match internal_run_pipeline(&app) {
            Ok(_) => {
                log::info!("Pipeline completed successfully");
            }
            Err(e) => {
                log::error!("Pipeline failed: {}", e);
                // Emit error event to frontend
                app.emit("pipeline-error", e.clone()).unwrap_or_default();
                play_sound("Click"); // Error sound
            }
        }
        // Go back to idle after completion or error
        set_status(&app, AppStatus::Idle);
    });
}

fn internal_run_pipeline(app: &AppHandle) -> Result<(), String> {
    log::info!("Starting pipeline...");

    // 1. Get Paths and Ensure Model
    let instruction_path = app.path().app_data_dir().unwrap().join("instruction.wav");
    let content_path = app.path().app_data_dir().unwrap().join("content.wav");
    let model_path = app.path().app_data_dir().unwrap().join(MODEL_FILENAME);

    log::info!("Checking for Whisper model...");
    ensure_whisper_model(app, &model_path)?;

    // 2. Load Settings & Model
    log::info!("Loading settings...");
    let data = load_data(app)?;
    let model = data.settings.model.clone();

    // Load Whisper Context (can take some time, maybe cache this in state later if slow)
    // For now simple load on demand
    log::info!("Loading Whisper model into memory...");
    app.emit("pipeline-status", "Loading AI model...")
        .unwrap_or_default();
    let ctx = WhisperContext::new_with_params(
        &model_path.to_string_lossy(),
        WhisperContextParameters::default(),
    )
    .map_err(|e| format!("Failed to load Whisper model: {}", e))?;

    // 3. Transcribe Instruction
    log::info!("Transcribing instruction...");
    let instruction_text = transcribe_local(&ctx, &instruction_path)?;
    log::info!("Instruction: {}", instruction_text);

    // 4. Transcribe Content
    log::info!("Transcribing content...");
    let content_text = transcribe_local(&ctx, &content_path)?;
    log::info!("Content: {}", content_text);

    // 5. Enrich
    log::info!("Enriching with model: {}", model);
    let enriched_text = enrich(&instruction_text, &content_text, &model)?;
    log::info!("Enrichment complete.");

    // 6. Clipboard
    app.clipboard()
        .write_text(enriched_text.clone())
        .map_err(|e| format!("Clipboard error: {}", e))?;

    // 7. Save History
    let item = HistoryItem {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        instruction: instruction_text,
        original_content: content_text,
        enriched_content: enriched_text,
    };

    let mut current_data = load_data(app)?;
    current_data.history.push(item);
    save_data(app, &current_data)?;

    // 8. Success
    play_sound("Success");
    app.emit("pipeline-complete", "success").unwrap_or_default();

    Ok(())
}

fn ensure_whisper_model(app: &AppHandle, path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        log::info!("Whisper model found at {:?}", path);
        return Ok(());
    }

    log::info!("Whisper model not found. Downloading to {:?}", path);
    log::info!("This may take several minutes (model is ~1.5GB)...");

    // Emit status to UI
    app.emit(
        "pipeline-status",
        "Downloading AI model (this may take several minutes)...",
    )
    .unwrap_or_default();

    // Create dir if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create model directory: {}. Please check write permissions.",
                e
            )
        })?;
    }

    let mut response = Client::new()
        .get(MODEL_URL)
        .timeout(std::time::Duration::from_secs(600)) // 10 minute timeout for large file
        .send()
        .map_err(|e| format!("Failed to download Whisper model. Please check your internet connection. Error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download Whisper model. Server returned status: {}. Please try again later.",
            response.status()
        ));
    }

    let mut file = File::create(path).map_err(|e| {
        format!(
            "Failed to create model file. Please check write permissions. Error: {}",
            e
        )
    })?;

    response.copy_to(&mut file)
        .map_err(|e| format!("Failed to save model file. Please ensure you have enough disk space (~1.5GB). Error: {}", e))?;

    log::info!("Whisper model download complete!");
    Ok(())
}

fn transcribe_local(ctx: &WhisperContext, path: &PathBuf) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File not found: {:?}", path));
    }

    // Read WAV and convert to f32 16kHz mono
    // Note: audio.rs should now try to record 16kHz.
    // If it recorded at another rate, this simple reader will fail context expectations
    // because Whisper expects 16k.

    let mut reader = hound::WavReader::open(path).map_err(|e| e.to_string())?;
    let spec = reader.spec();
    log::info!(
        "Audio file spec: {} channels, {} Hz, {:?} format",
        spec.channels,
        spec.sample_rate,
        spec.sample_format
    );

    // Convert samples to f32
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => reader
            .samples::<i16>()
            .map(|s| s.map(|x| x as f32 / 32768.0).unwrap_or(0.0))
            .collect(),
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect(),
    };

    // If stereo (channels=2), filter to mono (take every 2nd sample or average)
    let mut mono_samples: Vec<f32> = if spec.channels == 2 {
        samples
            .chunks(2)
            .map(|chunk| (chunk[0] + chunk[1]) / 2.0)
            .collect()
    } else if spec.channels == 1 {
        samples
    } else {
        // Fallback for > 2 channels: just take first channel
        samples
            .chunks(spec.channels as usize)
            .map(|chunk| chunk[0])
            .collect()
    };

    // Resample if needed
    if spec.sample_rate != 16000 {
        log::info!("Resampling from {} Hz to 16000 Hz", spec.sample_rate);
        mono_samples = resample_linear(&mono_samples, spec.sample_rate, 16000);
    }

    log::info!("Creating Whisper inference state...");
    // Create state
    let mut state = ctx
        .create_state()
        .map_err(|e| format!("Failed to create Whisper state: {}", e))?;

    log::info!("Configuring Whisper parameters...");
    // Set params
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Use 4 threads for better performance
    params.set_n_threads(4);

    // Use "auto" for automatic language detection
    params.set_language(Some("auto"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    log::info!(
        "Running Whisper inference with auto language detection on {} samples...",
        mono_samples.len()
    );
    // Run
    state
        .full(params, &mono_samples[..])
        .map_err(|e| format!("failed to run model: {}", e))?;

    log::info!("Whisper inference complete, extracting text...");

    // Collect text
    let num_segments = state.full_n_segments();
    let mut text = String::new();
    for i in 0..num_segments {
        if let Some(segment) = state.get_segment(i) {
            if let Ok(s) = segment.to_str() {
                text.push_str(s);
                text.push(' ');
            }
        }
    }

    Ok(text.trim().to_string())
}

fn enrich(instruction: &str, content: &str, model: &str) -> Result<String, String> {
    let prompt = format!(
        "Instruction: {}\nOriginal Content: {}\n\nPlease rewrite the content following the instruction. Return ONLY the rewritten text, nothing else.",
        instruction, content
    );

    let client = Client::new();
    let res = client
        .post(OLLAMA_API_URL)
        .timeout(std::time::Duration::from_secs(120)) // 2 minute timeout
        .json(&json!({
            "model": model,
            "stream": false,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .map_err(|e| {
            format!(
                "Failed to connect to Ollama at {}. Please ensure Ollama is running. Error: {}",
                OLLAMA_API_URL, e
            )
        })?;

    if !res.status().is_success() {
        return Err(format!("Ollama error (status {}). Model '{}' may not be installed. Please run 'ollama pull {}' in your terminal.", res.status(), model, model));
    }

    let body: Value = res
        .json()
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let result = body["message"]["content"]
        .as_str()
        .ok_or("Invalid response format from Ollama. Please check Ollama version and model compatibility.")?
        .trim()
        .to_string();

    Ok(result)
}

fn resample_linear(input: &[f32], input_rate: u32, target_rate: u32) -> Vec<f32> {
    // Handle edge cases
    if input.is_empty() {
        log::warn!("Resampling empty input");
        return Vec::new();
    }

    if input_rate == target_rate {
        log::info!("No resampling needed (same rate)");
        return input.to_vec();
    }

    let ratio = input_rate as f64 / target_rate as f64;
    let new_len = (input.len() as f64 / ratio).ceil() as usize;

    log::info!(
        "Resampling {} samples -> {} samples (ratio: {:.2})",
        input.len(),
        new_len,
        ratio
    );

    let mut output = Vec::with_capacity(new_len);
    let input_len = input.len();
    let input_len_minus_1 = input_len.saturating_sub(1);

    for i in 0..new_len {
        let input_idx = i as f64 * ratio;
        let idx0 = input_idx.floor() as usize;

        // Bounds check
        if idx0 >= input_len {
            break;
        }

        let idx1 = (idx0 + 1).min(input_len_minus_1);
        let t = (input_idx - input_idx.floor()) as f32;

        // Linear interpolation
        let val = input[idx0] * (1.0 - t) + input[idx1] * t;
        output.push(val);
    }

    log::info!("Resampling complete: generated {} samples", output.len());
    output
}
