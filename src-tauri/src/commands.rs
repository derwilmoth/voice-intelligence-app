use crate::audio::{list_input_devices, play_sound};
use crate::models::{AppStateData, HistoryItem, Settings};
use crate::ollama::scan_models;
use crate::store::{load_data, save_data};
use tauri::AppHandle;
#[cfg(desktop)]
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[tauri::command]
pub fn get_models() -> Vec<String> {
    scan_models()
}

#[tauri::command]
pub fn get_input_devices() -> Vec<String> {
    list_input_devices()
}

#[tauri::command]
pub fn play_test_sound(name: String) {
    play_sound(&name);
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, String> {
    let data = load_data(&app)?;
    Ok(data.settings)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    #[cfg(desktop)]
    {
        // Unregister all existing shortcuts
        if let Err(e) = app.global_shortcut().unregister_all() {
            log::error!("Failed to unregister hotkeys: {}", e);
        }
        // Register the new hotkey
        if let Err(e) = app.global_shortcut().register(settings.hotkey.as_str()) {
            return Err(format!(
                "Failed to register hotkey '{}': {}",
                settings.hotkey, e
            ));
        }
    }

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
