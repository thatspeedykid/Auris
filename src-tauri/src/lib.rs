// Auris — by PrivacyChase
// AGPL v3.0 — https://github.com/thatspeedykid/Auris

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod dsp;
mod eq;
mod privacy;
mod profiles;

use std::sync::Arc;
use tauri::Manager;
use serde::{Serialize, Deserialize};

// Global shared audio state — lives for the lifetime of the app
struct AppState {
    shared: audio::SharedState,
    engine_started: std::sync::atomic::AtomicBool,
}

// ── DSP status ────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_dsp_status() -> String {
    dsp::get_status_string()
}

// ── Audio engine control ──────────────────────────────────────────────────────

#[tauri::command]
fn start_audio_engine(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    if state.engine_started.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok("already_running".to_string());
    }

    let profile = eq::beats_studio_pro_profile();
    audio::start_engine(profile, state.shared.clone())?;
    state.engine_started.store(true, std::sync::atomic::Ordering::Relaxed);
    Ok("started".to_string())
}

#[tauri::command]
fn stop_audio_engine(state: tauri::State<Arc<AppState>>) {
    audio::stop_engine(&state.shared);
    state.engine_started.store(false, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
fn set_eq_enabled(enabled: bool, state: tauri::State<Arc<AppState>>) {
    state.shared.eq_enabled.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
fn set_desser_enabled(enabled: bool, state: tauri::State<Arc<AppState>>) {
    state.shared.desser_enabled.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
fn get_engine_status(state: tauri::State<Arc<AppState>>) -> audio::AudioEngineState {
    audio::AudioEngineState {
        running: state.engine_started.load(std::sync::atomic::Ordering::Relaxed),
        eq_enabled: state.shared.eq_enabled.load(std::sync::atomic::Ordering::Relaxed),
        desser_enabled: state.shared.desser_enabled.load(std::sync::atomic::Ordering::Relaxed),
        ..Default::default()
    }
}

// ── EQ profiles ───────────────────────────────────────────────────────────────

#[tauri::command]
fn get_headphone_profiles() -> Vec<eq::HeadphoneProfile> {
    eq::built_in_profiles()
}

#[tauri::command]
fn get_active_profile() -> eq::HeadphoneProfile {
    eq::beats_studio_pro_profile()
}

// ── App profiles + presets ────────────────────────────────────────────────────

#[tauri::command]
fn get_app_profiles() -> Vec<profiles::AppProfile> {
    profiles::default_app_profiles()
}

#[tauri::command]
fn get_presets() -> Vec<profiles::Preset> {
    profiles::built_in_presets()
}

// ── Privacy + version ─────────────────────────────────────────────────────────

#[tauri::command]
fn get_privacy_log() -> Vec<String> {
    privacy::get_log()
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

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

// ── App entry ─────────────────────────────────────────────────────────────────

pub fn run() {
    env_logger::init();

    let app_state = Arc::new(AppState {
        shared: Arc::new(audio::SharedAudioState::new()),
        engine_started: std::sync::atomic::AtomicBool::new(false),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_dsp_status,
            start_audio_engine,
            stop_audio_engine,
            set_eq_enabled,
            set_desser_enabled,
            get_engine_status,
            get_headphone_profiles,
            get_active_profile,
            get_app_profiles,
            get_presets,
            get_privacy_log,
            get_version,
            get_eq_state,
        ])
        .setup(|app| {
            // Auto-start the audio engine on launch
            let state = app.state::<Arc<AppState>>();
            let profile = eq::beats_studio_pro_profile();
            match audio::start_engine(profile, state.shared.clone()) {
                Ok(_) => {
                    state.engine_started.store(true, std::sync::atomic::Ordering::Relaxed);
                    log::info!("Auris: audio engine started on launch");
                }
                Err(e) => log::warn!("Auris: audio engine failed to start: {}", e),
            }

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
