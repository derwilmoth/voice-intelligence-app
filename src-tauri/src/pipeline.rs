use crate::audio::play_sound;
use crate::logic::{set_status, AppStatus};
use crate::models::HistoryItem;
use crate::store::{load_data, save_data};
use base64::Engine;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

const OLLAMA_API_URL: &str = "http://localhost:11434/api/chat";
const WHISPER_MODEL: &str = "karanchopda333/whisper";

pub fn run_pipeline(app: AppHandle) {
    std::thread::spawn(move || {
        let _ = internal_run_pipeline(&app);
        // Regardless of success/fail, we go back to idle eventually?
        // Or if fail, maybe error state. For now idle.
        set_status(&app, AppStatus::Idle);
    });
}

fn internal_run_pipeline(app: &AppHandle) -> Result<(), String> {
    // 1. Get Paths
    let instruction_path = app.path().app_data_dir().unwrap().join("instruction.wav");
    let content_path = app.path().app_data_dir().unwrap().join("content.wav");

    // 2. Load Settings
    let data = load_data(app)?;
    let model = data.settings.model.clone();

    // 3. Transcribe Instruction
    log::info!("Transcribing instruction...");
    let instruction_text = transcribe(&instruction_path)?;
    log::info!("Instruction: {}", instruction_text);

    // 4. Transcribe Content
    log::info!("Transcribing content...");
    let content_text = transcribe(&content_path)?;
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

    // We need to load data again to append, avoiding overwrites if possible,
    // but here we are in a single thread flow regarding data usually.
    // Ideally use a database or improved store.rs, but reusing load/save is fine for this scale.
    let mut current_data = load_data(app)?;
    current_data.history.push(item);
    save_data(app, &current_data)?;

    // 8. Success
    play_sound("Success");
    app.emit("pipeline-complete", "success").unwrap_or_default();

    Ok(())
}

fn transcribe(path: &PathBuf) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File not found: {:?}", path));
    }

    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    // Encode audio to base64
    // NOTE: karanchopda333/whisper via Ollama might expect wrapping or specific prompt.
    // Based on community usage for whisper in ollama, usually one sends the filename if local,
    // but via API it usually expects base64 or blob. Use base64 string.
    let b64_audio = base64::engine::general_purpose::STANDARD.encode(&buffer);

    let client = Client::new();
    let res = client
        .post(OLLAMA_API_URL)
        .json(&json!({
            "model": WHISPER_MODEL,
            "stream": false,
            "messages": [
                {
                    "role": "user",
                    "content": b64_audio
                }
            ]
        }))
        .send()
        .map_err(|e| format!("Ollama request failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ollama error: {}", res.status()));
    }

    let body: Value = res.json().map_err(|e| e.to_string())?;

    let content = body["message"]["content"]
        .as_str()
        .ok_or("Invalid response format from Ollama")?
        .trim()
        .to_string();

    Ok(content)
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
