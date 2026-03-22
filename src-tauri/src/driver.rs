// Auris — Driver installer
// Silently installs the FxSound VAD (Virtual Audio Device) kernel driver
// which is bundled with Auris. This gives us a system-wide audio pipe
// without requiring the user to install FxSound.
//
// The driver files are signed by Microsoft (fxvadntamd64.cat) so Windows
// will accept them without test mode or any user prompts beyond UAC.

use std::process::Command;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DriverStatus {
    Installed,
    NotInstalled,
    Error(String),
}

/// Check if the FxSound VAD driver is already installed
pub fn check_driver_installed() -> DriverStatus {
    let output = Command::new("powershell")
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            "(Get-PnpDevice | Where-Object {$_.HardwareID -like '*fxvad*' -or $_.FriendlyName -like '*FxSound*'}).Status"
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if stdout == "OK" || stdout.contains("OK") {
                DriverStatus::Installed
            } else {
                DriverStatus::NotInstalled
            }
        }
        Err(e) => DriverStatus::Error(e.to_string()),
    }
}

/// Get the path to our bundled driver files
/// In dev: looks relative to the binary
/// In release: bundled in the installer, extracted to %APPDATA%\Auris\driver
fn get_driver_dir() -> PathBuf {
    // Try next to the executable first (dev mode)
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();

    let dev_path = exe_dir.join("..").join("..").join("..").join("driver");
    if dev_path.join("fxvad.inf").exists() {
        return dev_path.canonicalize().unwrap_or(dev_path);
    }

    // Release: %APPDATA%\Auris\driver
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join("Auris").join("driver")
}

/// Install the driver silently using DfxSetupDrv.exe
/// Requires elevation (UAC prompt will appear once on first install)
pub fn install_driver() -> Result<(), String> {
    let driver_dir = get_driver_dir();
    let setup_exe = driver_dir.join("DfxSetupDrv.exe");
    let inf_path = driver_dir.join("fxvad.inf");

    if !setup_exe.exists() {
        return Err(format!("Driver setup not found at: {}", setup_exe.display()));
    }

    log::info!("Auris: installing VAD driver from {}", driver_dir.display());

    // DfxSetupDrv.exe installs the driver — it handles the devcon logic internally
    let output = Command::new(&setup_exe)
        .args(["/install", &inf_path.to_string_lossy()])
        .output()
        .map_err(|e| format!("Failed to run driver installer: {e}"))?;

    if output.status.success() {
        log::info!("Auris: VAD driver installed successfully");
        Ok(())
    } else {
        // Also try direct devcon approach as fallback
        install_driver_devcon(&driver_dir)
    }
}

fn install_driver_devcon(driver_dir: &Path) -> Result<(), String> {
    let devcon = driver_dir.join("fxdevcon64.exe");
    let inf_path = driver_dir.join("fxvad.inf");

    if !devcon.exists() {
        return Err(format!("devcon not found at: {}", devcon.display()));
    }

    let output = Command::new(&devcon)
        .args(["install", &inf_path.to_string_lossy(), "Root\\fxvad"])
        .output()
        .map_err(|e| format!("devcon failed: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || stdout.contains("installed") || stdout.contains("updated") {
        log::info!("Auris: VAD driver installed via devcon");
        Ok(())
    } else {
        Err(format!("devcon error: {} {}", stdout.trim(), stderr.trim()))
    }
}

/// Full first-run check: install driver if not present
pub fn ensure_driver_installed() -> Result<bool, String> {
    match check_driver_installed() {
        DriverStatus::Installed => {
            log::info!("Auris: VAD driver already installed");
            Ok(false) // false = didn't need to install
        }
        DriverStatus::NotInstalled => {
            log::info!("Auris: VAD driver not found, installing...");
            install_driver()?;
            Ok(true) // true = just installed
        }
        DriverStatus::Error(e) => {
            log::warn!("Auris: Could not check driver status: {}", e);
            // Try installing anyway
            install_driver()?;
            Ok(true)
        }
    }
}
