use crate::audio::{play_sound, start_recording, stop_recording, AudioState};
use crate::store::{get_status, load_data, set_status as store_set_status};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppStatus {
    #[default]
    Idle,
    Instruction,
    Content,
    Processing,
}

impl AppStatus {
    fn as_str(&self) -> &'static str {
        match self {
            AppStatus::Idle => "idle",
            AppStatus::Instruction => "instruction",
            AppStatus::Content => "content",
            AppStatus::Processing => "processing",
        }
    }
}

pub struct LogicState {
    pub status: Arc<Mutex<AppStatus>>,
}

impl Default for LogicState {
    fn default() -> Self {
        Self::new()
    }
}

impl LogicState {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(AppStatus::Idle)),
        }
    }
}

pub fn handle_trigger(app: &AppHandle) {
    let audio_state = app.state::<AudioState>();

    // Get current status from JSON
    let current_status_str = match get_status(app) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get status from JSON: {}", e);
            return;
        }
    };

    let current = match current_status_str.as_str() {
        "idle" => AppStatus::Idle,
        "instruction" => AppStatus::Instruction,
        "content" => AppStatus::Content,
        "processing" => AppStatus::Processing,
        _ => AppStatus::Idle,
    };

    let timeout_minutes = load_data(app)
        .map(|data| data.settings.recording_timeout_minutes)
        .unwrap_or(10);

    let new_status = match current {
        AppStatus::Idle => {
            // Idle -> Instruction
            play_sound("Ping");

            // Start Recording 1
            let path = get_audio_path(app, "instruction.wav");
            let _ = start_recording(&audio_state, None, path, timeout_minutes, app.clone());

            AppStatus::Instruction
        }
        AppStatus::Instruction => {
            // Instruction -> Content
            play_sound("Click");

            // Stop Recording 1
            stop_recording(&audio_state);

            // Start Recording 2
            let path = get_audio_path(app, "content.wav");
            let _ = start_recording(&audio_state, None, path, timeout_minutes, app.clone());

            AppStatus::Content
        }
        AppStatus::Content => {
            // Content -> Processing
            play_sound("Swoosh");

            // Stop Recording 2
            stop_recording(&audio_state);

            // Trigger processing in background
            use crate::pipeline::run_pipeline;
            run_pipeline(app.clone());

            AppStatus::Processing
        }
        AppStatus::Processing => {
            // Ignore triggers while processing? Or cancel?
            // Let's ignore for now.
            return;
        }
    };

    set_status(app, new_status);
}

pub fn set_status(app: &AppHandle, new_status: AppStatus) {
    let status_str = new_status.as_str();
    // Update status in JSON
    if let Err(e) = store_set_status(app, status_str) {
        log::error!("Failed to save status to JSON: {}", e);
    }
    // Emit event to notify frontend
    let _ = app.emit("status-changed", status_str);
}

fn get_audio_path(app: &AppHandle, filename: &str) -> PathBuf {
    app.path().app_data_dir().unwrap().join(filename)
}
