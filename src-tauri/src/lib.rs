pub mod audio;
pub mod commands;
pub mod logic;
pub mod models;
pub mod ollama;
pub mod pipeline;
pub mod store;

use audio::AudioState;
use logic::LogicState;
use tauri::Manager;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[tauri::command]
fn manual_trigger(app: tauri::AppHandle) {
    logic::handle_trigger(&app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
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
            commands::get_app_info,
            manual_trigger
        ])
        .setup(|app| {
            // Enable logging in both debug and release builds
            // In release, logs go to a file in the app data directory
            let log_plugin = if cfg!(debug_assertions) {
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build()
            } else {
                // Release mode: write logs to file
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .target(tauri_plugin_log::Target::new(
                        tauri_plugin_log::TargetKind::LogDir {
                            file_name: Some("app.log".to_string()),
                        },
                    ))
                    .build()
            };
            app.handle().plugin(log_plugin)?;

            log::info!("=== Application Starting ===");
            log::info!("App data dir: {:?}", app.path().app_data_dir());
            log::info!(
                "Build mode: {}",
                if cfg!(debug_assertions) {
                    "DEBUG"
                } else {
                    "RELEASE"
                }
            );

            #[cfg(desktop)]
            {
                let handle = app.handle();
                handle.plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(|app, _shortcut, event| {
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

                // Tray Icon Setup
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let show_i = MenuItem::with_id(app, "show", "Show UI", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .icon(app.default_window_icon().unwrap().clone())
                    .build(app)?;
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
