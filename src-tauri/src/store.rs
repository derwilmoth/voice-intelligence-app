use crate::models::{AppStateData, HistoryItem, Settings};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime}; // Assuming models.rs is visible

const DATA_FILENAME: &str = "app_data.json";

fn get_data_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join(DATA_FILENAME)
}

pub fn save_data<R: Runtime>(app: &AppHandle<R>, data: &AppStateData) -> Result<(), String> {
    let path = get_data_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_data<R: Runtime>(app: &AppHandle<R>) -> Result<AppStateData, String> {
    let path = get_data_path(app);
    if !path.exists() {
        return Ok(AppStateData::default());
    }
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let data: AppStateData = serde_json::from_reader(file).unwrap_or_default();
    Ok(data)
}
