// DSP bridge — Phase 0
// Detects whether the FxSound virtual audio driver is installed and active
// by scanning Windows audio endpoints for the FxSound device.
//
// Phase 1 will add full FFI into DfxDsp.lib for actual audio processing.

use std::process::Command;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DspStatus {
    Active,
    Inactive,
    DriverMissing,
    Error(String),
}

/// Check if the FxSound virtual audio driver device is present on this system.
/// We do this by querying the Windows audio device list via PowerShell —
/// no unsafe FFI needed for Phase 0.
pub fn get_status() -> DspStatus {
    match check_fxsound_driver() {
        Ok(true)  => DspStatus::Active,
        Ok(false) => DspStatus::DriverMissing,
        Err(e)    => DspStatus::Error(e),
    }
}

fn check_fxsound_driver() -> Result<bool, String> {
    // Query Windows audio devices for anything named "FxSound"
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-WmiObject Win32_SoundDevice | Select-Object -ExpandProperty Name",
        ])
        .output()
        .map_err(|e| format!("PowerShell query failed: {e}"))?;

    if !output.status.success() {
        return Err("PowerShell returned non-zero exit code".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    Ok(stdout.contains("fxsound"))
}

/// Return a human-readable status string for the React UI
pub fn get_status_string() -> String {
    match get_status() {
        DspStatus::Active        => "active".to_string(),
        DspStatus::Inactive      => "inactive".to_string(),
        DspStatus::DriverMissing => "driver_missing".to_string(),
        DspStatus::Error(e)      => format!("error:{e}"),
    }
}

pub fn set_enabled(_enabled: bool) -> Result<(), String> {
    // TODO Phase 1: toggle FxSound DSP processing on/off via FFI
    Err("DSP FFI not yet wired — coming in Phase 1".to_string())
}
