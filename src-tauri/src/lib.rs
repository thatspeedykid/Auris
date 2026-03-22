// Auris — by PrivacyChase
// AGPL v3.0 — https://github.com/thatspeedykid/Auris

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;
mod eq;
mod privacy;
mod profiles;

use tauri::Manager;
use serde::{Serialize, Deserialize};

// ── DSP status ────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_dsp_status() -> String {
    dsp::get_status_string()
}

// ── EQ profiles ───────────────────────────────────────────────────────────────

#[tauri::command]
fn get_headphone_profiles() -> Vec<eq::HeadphoneProfile> {
    eq::built_in_profiles()
}

#[tauri::command]
fn get_active_profile() -> eq::HeadphoneProfile {
    // In Phase 1: detect connected headphone from WASAPI device name
    // For now: return Beats Studio Pro as default (since that's what you're wearing)
    eq::beats_studio_pro_profile()
}

// ── App profiles ──────────────────────────────────────────────────────────────

#[tauri::command]
fn get_app_profiles() -> Vec<profiles::AppProfile> {
    profiles::default_app_profiles()
}

#[tauri::command]
fn get_presets() -> Vec<profiles::Preset> {
    profiles::built_in_presets()
}

// ── Privacy log ───────────────────────────────────────────────────────────────

#[tauri::command]
fn get_privacy_log() -> Vec<String> {
    privacy::get_log()
}

// ── Version ───────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ── EQ state (in-memory for Phase 1) ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EqState {
    enabled: bool,
    active_profile: String,
    active_preset: String,
    desser_enabled: bool,
    desser_strength: f64,
}

#[tauri::command]
fn get_eq_state() -> EqState {
    EqState {
        enabled: true,
        active_profile: "Beats Studio Pro".to_string(),
        active_preset: "headphone".to_string(),
        desser_enabled: true,
        desser_strength: 1.2,
    }
}

#[tauri::command]
fn set_eq_enabled(enabled: bool) -> Result<(), String> {
    // Phase 2: wire to WASAPI audio processing thread
    let _ = enabled;
    Ok(())
}

// ── App entry ─────────────────────────────────────────────────────────────────

pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_dsp_status,
            get_headphone_profiles,
            get_active_profile,
            get_app_profiles,
            get_presets,
            get_privacy_log,
            get_version,
            get_eq_state,
            set_eq_enabled,
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
