// Auris — Per-app audio profiles
// Maps Windows process names to EQ presets

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProfile {
    pub app_name: String,
    pub process_names: Vec<String>, // exe names to match
    pub preset: String,
    pub desser_enabled: bool,
    pub desser_strength: f64, // 1.0 = normal, 2.0 = aggressive
}

pub fn default_app_profiles() -> Vec<AppProfile> {
    vec![
        AppProfile {
            app_name: "YouTube / Chrome".to_string(),
            process_names: vec!["chrome.exe".to_string(), "msedge.exe".to_string(), "firefox.exe".to_string()],
            preset: "headphone".to_string(),
            desser_enabled: true,   // kill that shhh on YouTube voices
            desser_strength: 1.2,
        },
        AppProfile {
            app_name: "Spotify".to_string(),
            process_names: vec!["Spotify.exe".to_string()],
            preset: "headphone".to_string(),
            desser_enabled: false,  // don't touch music
            desser_strength: 1.0,
        },
        AppProfile {
            app_name: "Discord".to_string(),
            process_names: vec!["Discord.exe".to_string()],
            preset: "voice".to_string(),
            desser_enabled: true,
            desser_strength: 1.5,   // aggressive on voice calls
        },
        AppProfile {
            app_name: "Games".to_string(),
            process_names: vec![], // catch-all for unknown processes
            preset: "gaming".to_string(),
            desser_enabled: false,
            desser_strength: 1.0,
        },
    ]
}

// Preset EQ flavors (applied on top of headphone correction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub description: String,
    pub bass_boost_db: f64,
    pub mid_db: f64,
    pub treble_db: f64,
}

pub fn built_in_presets() -> Vec<Preset> {
    vec![
        Preset { name: "headphone".to_string(), description: "Neutral, corrected".to_string(), bass_boost_db: 0.0, mid_db: 0.0, treble_db: 0.0 },
        Preset { name: "voice".to_string(), description: "Voice call clarity".to_string(), bass_boost_db: -2.0, mid_db: 2.5, treble_db: -1.5 },
        Preset { name: "gaming".to_string(), description: "Wide soundstage".to_string(), bass_boost_db: 1.5, mid_db: 0.5, treble_db: 2.0 },
        Preset { name: "bass_boost".to_string(), description: "Extra low end".to_string(), bass_boost_db: 4.0, mid_db: -1.0, treble_db: 0.0 },
        Preset { name: "podcast".to_string(), description: "Warm speech".to_string(), bass_boost_db: -1.0, mid_db: 3.0, treble_db: -2.0 },
        Preset { name: "classical".to_string(), description: "Concert hall".to_string(), bass_boost_db: -1.0, mid_db: 0.5, treble_db: 1.5 },
        Preset { name: "flat".to_string(), description: "No processing".to_string(), bass_boost_db: 0.0, mid_db: 0.0, treble_db: 0.0 },
    ]
}
