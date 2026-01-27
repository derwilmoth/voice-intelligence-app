// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Always hide console in release mode, even though we use debug settings for whisper-rs compatibility
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
#![cfg_attr(
    all(target_os = "windows", debug_assertions),
    windows_subsystem = "windows"
)]

fn main() {
    app_lib::run();
}
