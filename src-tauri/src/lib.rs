pub mod audio;
pub mod commands;
pub mod logic;
pub mod models;
pub mod ollama;
pub mod store;

use audio::AudioState;
use logic::LogicState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[tauri::command]
fn manual_trigger(app: tauri::AppHandle) {
    logic::handle_trigger(&app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AudioState::new())
        .manage(LogicState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_models,
            commands::get_input_devices,
            commands::play_test_sound,
            commands::get_settings,
            commands::save_settings,
            commands::get_history,
            commands::add_history_item,
            commands::clear_history,
            manual_trigger
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(desktop)]
            {
                let handle = app.handle();
                handle.plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(|app, shortcut, event| {
                            if event.state() == ShortcutState::Pressed {
                                logic::handle_trigger(app);
                            }
                        })
                        .build(),
                )?;

                let data = store::load_data(handle).unwrap_or_default();
                if let Err(e) = handle
                    .global_shortcut()
                    .register(data.settings.hotkey.as_str())
                {
                    log::error!("Failed to register hotkey: {}", e);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
