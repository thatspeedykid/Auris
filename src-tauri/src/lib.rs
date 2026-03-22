// Auris — by PrivacyChase
// AGPL v3.0 — https://github.com/thatspeedykid/Auris

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;
mod privacy;

use tauri::Manager;

#[tauri::command]
fn get_dsp_status() -> String {
    // TODO Phase 0: wire to actual DSP state
    // dsp::get_status()
    "inactive".to_string()
}

#[tauri::command]
fn get_privacy_log() -> Vec<String> {
    privacy::get_log()
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_dsp_status,
            get_privacy_log,
            get_version,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Auris");
}
