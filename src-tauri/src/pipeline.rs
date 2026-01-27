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
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin";
const MODEL_FILENAME: &str = "ggml-large-v3.bin";

pub fn run_pipeline(app: AppHandle) {
    std::thread::spawn(move || {
        let _ = internal_run_pipeline(&app);
        // Regardless of success/fail, we go back to idle eventually
        set_status(&app, AppStatus::Idle);
    });
}

fn internal_run_pipeline(app: &AppHandle) -> Result<(), String> {
    // 1. Get Paths and Ensure Model
    let instruction_path = app.path().app_data_dir().unwrap().join("instruction.wav");
    let content_path = app.path().app_data_dir().unwrap().join("content.wav");
    let model_path = app.path().app_data_dir().unwrap().join(MODEL_FILENAME);

    ensure_whisper_model(&model_path)?;

    // 2. Load Settings & Model
    let data = load_data(app)?;
    let model = data.settings.model.clone();

    // Load Whisper Context (can take some time, maybe cache this in state later if slow)
    // For now simple load on demand
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

fn ensure_whisper_model(path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    log::info!("Downloading Whisper model to {:?}", path);
    // Create dir if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut response = Client::new()
        .get(MODEL_URL)
        .send()
        .map_err(|e| format!("Failed to download model: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download model, status: {}",
            response.status()
        ));
    }

    let mut file = File::create(path).map_err(|e| e.to_string())?;
    response.copy_to(&mut file).map_err(|e| e.to_string())?;

    log::info!("Model download complete.");
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

    // Create state
    let mut state = ctx
        .create_state()
        .map_err(|e| format!("Failed to create Whisper state: {}", e))?;

    // Set params
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(4);
    params.set_language(Some("auto"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // Run
    state
        .full(params, &mono_samples[..])
        .map_err(|e| format!("failed to run model: {}", e))?;

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
        .map_err(|e| format!("Ollama enrichment failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ollama error: {}", res.status()));
    }

    let body: Value = res.json().map_err(|e| e.to_string())?;

    let result = body["message"]["content"]
        .as_str()
        .ok_or("Invalid response format from Ollama")?
        .trim()
        .to_string();

    Ok(result)
}

fn resample_linear(input: &[f32], input_rate: u32, target_rate: u32) -> Vec<f32> {
    let ratio = input_rate as f32 / target_rate as f32;
    // Calculate new length
    let new_len = (input.len() as f32 / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(new_len);

    for i in 0..new_len {
        let input_idx = i as f32 * ratio;
        let idx0 = input_idx.floor() as usize;
        let idx1 = (idx0 + 1).min(input.len() - 1);
        let t = input_idx - input_idx.floor();

        // Linear interpolation
        // Safety check for empty input
        if idx0 < input.len() {
            let val = input[idx0] * (1.0 - t) + input[idx1] * t;
            output.push(val);
        }
    }
    output
}
