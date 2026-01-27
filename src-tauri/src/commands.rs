use crate::models::{AppStateData, HistoryItem, Settings};
use crate::store::{load_data, save_data};
use tauri::AppHandle;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, String> {
    let data = load_data(&app)?;
    Ok(data.settings)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let mut data = load_data(&app)?;
    data.settings = settings;
    save_data(&app, &data)
}

#[tauri::command]
pub fn get_history(app: AppHandle) -> Result<Vec<HistoryItem>, String> {
    let data = load_data(&app)?;
    Ok(data.history)
}

// Internal helper might be needed later, but for now specific command:
#[tauri::command]
pub fn add_history_item(app: AppHandle, item: HistoryItem) -> Result<(), String> {
    let mut data = load_data(&app)?;
    data.history.push(item);
    save_data(&app, &data)
}

#[tauri::command]
pub fn clear_history(app: AppHandle) -> Result<(), String> {
    let mut data = load_data(&app)?;
    data.history.clear();
    save_data(&app, &data)
}
