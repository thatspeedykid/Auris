// Privacy audit log — Auris core promise:
// This log should ALWAYS be empty. Any entry here is a bug.
//
// In production this tracks every attempted network call,
// file access outside %APPDATA%\PrivacyChase\Auris,
// and any system API that could exfiltrate data.

use std::sync::Mutex;

static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn get_log() -> Vec<String> {
    LOG.lock().unwrap().clone()
}

/// Record a privacy event. Called if something unexpected happens.
/// The existence of any entry in this log is a bug to be fixed.
#[allow(dead_code)]
pub fn record(event: &str) {
    let mut log = LOG.lock().unwrap();
    let entry = format!(
        "[{}] {}",
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
        event
    );
    log::warn!("PRIVACY EVENT: {}", entry);
    log.push(entry);
}
