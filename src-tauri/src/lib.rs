// Auris — by PrivacyChase
// AGPL v3.0 — https://github.com/thatspeedykid/Auris

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod driver;
mod dsp;
mod eq;
mod fxsound_bridge;
mod privacy;
mod profiles;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tauri::Manager;
use serde::{Serialize, Deserialize};

struct AppState {
    eq_enabled: AtomicBool,
    desser_enabled: AtomicBool,
    volume_db: std::sync::Mutex<f64>,
    driver_installed: AtomicBool,
}

impl AppState {
    fn new() -> Self {
        Self {
            eq_enabled: AtomicBool::new(true),
            desser_enabled: AtomicBool::new(true),
            volume_db: std::sync::Mutex::new(0.0),
            driver_installed: AtomicBool::new(false),
        }
    }
}

// ── Driver ────────────────────────────────────────────────────────────────────

#[tauri::command]
fn install_driver(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    match driver::ensure_driver_installed() {
        Ok(installed) => {
            state.driver_installed.store(true, Ordering::Relaxed);
            if installed {
                Ok("installed".to_string())
            } else {
                Ok("already_installed".to_string())
            }
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
fn get_driver_status() -> String {
    match driver::check_driver_installed() {
        driver::DriverStatus::Installed    => "installed".to_string(),
        driver::DriverStatus::NotInstalled => "not_installed".to_string(),
        driver::DriverStatus::Error(e)     => format!("error:{e}"),
    }
}

// ── DSP status ────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_dsp_status() -> String {
    dsp::get_status_string()
}

// ── EQ control ────────────────────────────────────────────────────────────────

#[tauri::command]
fn set_eq_enabled(enabled: bool, state: tauri::State<Arc<AppState>>) -> Result<(), String> {
    state.eq_enabled.store(enabled, Ordering::Relaxed);
    let profile = eq::beats_studio_pro_profile();
    let vol = *state.volume_db.lock().unwrap();
    fxsound_bridge::push_profile_to_fxsound(&profile.filters, profile.preamp_db, enabled, vol)
}

#[tauri::command]
fn set_desser_enabled(enabled: bool, state: tauri::State<Arc<AppState>>) {
    state.desser_enabled.store(enabled, Ordering::Relaxed);
}

#[tauri::command]
fn set_volume(db: f64, state: tauri::State<Arc<AppState>>) -> Result<(), String> {
    *state.volume_db.lock().unwrap() = db;
    let profile = eq::beats_studio_pro_profile();
    let enabled = state.eq_enabled.load(Ordering::Relaxed);
    fxsound_bridge::push_profile_to_fxsound(&profile.filters, profile.preamp_db, enabled, db)
}

#[tauri::command]
fn apply_profile(state: tauri::State<Arc<AppState>>) -> Result<(), String> {
    let profile = eq::beats_studio_pro_profile();
    let enabled = state.eq_enabled.load(Ordering::Relaxed);
    let vol = *state.volume_db.lock().unwrap();
    fxsound_bridge::push_profile_to_fxsound(&profile.filters, profile.preamp_db, enabled, vol)
}

// ── Queries ───────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_headphone_profiles() -> Vec<eq::HeadphoneProfile> {
    eq::built_in_profiles()
}

#[tauri::command]
fn get_active_profile() -> eq::HeadphoneProfile {
    eq::beats_studio_pro_profile()
}

#[tauri::command]
fn get_app_profiles() -> Vec<profiles::AppProfile> {
    profiles::default_app_profiles()
}

#[tauri::command]
fn get_presets() -> Vec<profiles::Preset> {
    profiles::built_in_presets()
}

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
    volume_db: f64,
    driver_installed: bool,
}

#[tauri::command]
fn get_eq_state(state: tauri::State<Arc<AppState>>) -> EqState {
    EqState {
        enabled: state.eq_enabled.load(Ordering::Relaxed),
        active_profile: "Beats Studio Pro".to_string(),
        active_preset: "headphone".to_string(),
        desser_enabled: state.desser_enabled.load(Ordering::Relaxed),
        volume_db: *state.volume_db.lock().unwrap(),
        driver_installed: state.driver_installed.load(Ordering::Relaxed),
    }
}

// ── App entry ─────────────────────────────────────────────────────────────────

pub fn run() {
    env_logger::init();

    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            install_driver,
            get_driver_status,
            get_dsp_status,
            set_eq_enabled,
            set_desser_enabled,
            set_volume,
            apply_profile,
            get_headphone_profiles,
            get_active_profile,
            get_app_profiles,
            get_presets,
            get_privacy_log,
            get_version,
            get_eq_state,
        ])
        .setup(|app| {
            let state = app.state::<Arc<AppState>>();

            // Check driver on launch, install if missing
            std::thread::spawn({
                let state = state.inner().clone();
                move || {
                    match driver::ensure_driver_installed() {
                        Ok(_) => {
                            state.driver_installed.store(true, Ordering::Relaxed);
                            // Push EQ profile now that driver is confirmed
                            let profile = eq::beats_studio_pro_profile();
                            let _ = fxsound_bridge::push_profile_to_fxsound(
                                &profile.filters, profile.preamp_db, true, 0.0
                            );
                            log::info!("Auris: driver ready, EQ profile pushed");
                        }
                        Err(e) => log::warn!("Auris: driver setup failed: {}", e),
                    }
                }
            });

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
