use crate::audio::{play_sound, start_recording, stop_recording, AudioState};
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

impl LogicState {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(AppStatus::Idle)),
        }
    }
}

pub fn handle_trigger(app: &AppHandle) {
    let logic_state = app.state::<LogicState>();
    let audio_state = app.state::<AudioState>();

    let mut status = logic_state.status.lock().unwrap();
    let current = *status;

    let new_status = match current {
        AppStatus::Idle => {
            // Idle -> Instruction
            play_sound("Ping");

            // Start Recording 1
            let path = get_audio_path(app, "instruction.wav");
            // Use default device from settings? For now default.
            // Ideally we read settings here.
            let _ = start_recording(&audio_state, None, path);

            AppStatus::Instruction
        }
        AppStatus::Instruction => {
            // Instruction -> Content
            play_sound("Click");

            // Stop Recording 1
            stop_recording(&audio_state);

            // Start Recording 2
            let path = get_audio_path(app, "content.wav");
            let _ = start_recording(&audio_state, None, path);

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

    *status = new_status;
    let _ = app.emit("status-changed", new_status.as_str());
}

pub fn set_status(app: &AppHandle, new_status: AppStatus) {
    let logic_state = app.state::<LogicState>();
    let mut status = logic_state.status.lock().unwrap();
    *status = new_status;
    let _ = app.emit("status-changed", new_status.as_str());
}

fn get_audio_path(app: &AppHandle, filename: &str) -> PathBuf {
    app.path().app_data_dir().unwrap().join(filename)
}
