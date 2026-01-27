pub mod audio;
pub mod commands;
pub mod models;
pub mod ollama;
pub mod store;

use audio::AudioState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AudioState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_models,
            commands::get_input_devices,
            commands::play_test_sound,
            commands::get_settings,
            commands::save_settings,
            commands::get_history,
            commands::add_history_item,
            commands::clear_history
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
