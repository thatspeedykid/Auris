// Auris — FxSound registry bridge
// Converts our parametric biquad EQ filters into FxSound's 31-band graphic EQ
// by writing directly to the registry keys FxSound reads.
//
// FxSound polls these registry values and applies them via its APO driver,
// which is already processing all system audio. We just tell it what to do.

use std::process::Command;
use crate::eq::{EqFilter, FilterType};

// FxSound uses 31 bands at standard ISO frequencies
const ISO_31_BANDS: [f64; 31] = [
    20.0, 25.0, 31.5, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0, 160.0,
    200.0, 250.0, 315.0, 400.0, 500.0, 630.0, 800.0, 1000.0, 1250.0, 1600.0,
    2000.0, 2500.0, 3150.0, 4000.0, 5000.0, 6300.0, 8000.0, 10000.0, 12500.0, 16000.0,
    20000.0,
];

// Evaluate a single parametric biquad filter's gain at a given frequency (dB)
fn biquad_gain_at_freq(filter: &EqFilter, freq: f64) -> f64 {
    if !filter.enabled { return 0.0; }

    use std::f64::consts::PI;

    // Using the magnitude response formula for biquad filters
    let w = 2.0 * PI * freq / 48000.0; // assume 48kHz
    let cos_w = w.cos();
    let sin_w = w.sin();
    let alpha = sin_w / (2.0 * filter.q);
    let a = 10f64.powf(filter.gain / 40.0);

    let (b0, b1, b2, a0, a1, a2) = match filter.filter_type {
        FilterType::Peaking => (
            1.0 + alpha * a,
            -2.0 * cos_w,
            1.0 - alpha * a,
            1.0 + alpha / a,
            -2.0 * cos_w,
            1.0 - alpha / a,
        ),
        FilterType::LowShelf => (
            a * ((a+1.0) - (a-1.0)*cos_w + 2.0*a.sqrt()*alpha),
            2.0 * a * ((a-1.0) - (a+1.0)*cos_w),
            a * ((a+1.0) - (a-1.0)*cos_w - 2.0*a.sqrt()*alpha),
            (a+1.0) + (a-1.0)*cos_w + 2.0*a.sqrt()*alpha,
            -2.0 * ((a-1.0) + (a+1.0)*cos_w),
            (a+1.0) + (a-1.0)*cos_w - 2.0*a.sqrt()*alpha,
        ),
        FilterType::HighShelf => (
            a * ((a+1.0) + (a-1.0)*cos_w + 2.0*a.sqrt()*alpha),
            -2.0 * a * ((a-1.0) + (a+1.0)*cos_w),
            a * ((a+1.0) + (a-1.0)*cos_w - 2.0*a.sqrt()*alpha),
            (a+1.0) - (a-1.0)*cos_w + 2.0*a.sqrt()*alpha,
            2.0 * ((a-1.0) - (a+1.0)*cos_w),
            (a+1.0) - (a-1.0)*cos_w - 2.0*a.sqrt()*alpha,
        ),
        _ => return 0.0,
    };

    // H(e^jw) magnitude in dB
    let b0n = b0 / a0; let b1n = b1 / a0; let b2n = b2 / a0;
    let a1n = a1 / a0; let a2n = a2 / a0;

    let num_re = b0n + b1n * cos_w + b2n * (2.0*cos_w*cos_w - 1.0);
    let num_im = -b1n * sin_w - b2n * 2.0 * sin_w * cos_w;
    let den_re = 1.0 + a1n * cos_w + a2n * (2.0*cos_w*cos_w - 1.0);
    let den_im = -a1n * sin_w - a2n * 2.0 * sin_w * cos_w;

    let num_mag2 = num_re * num_re + num_im * num_im;
    let den_mag2 = den_re * den_re + den_im * den_im;

    if den_mag2 < 1e-30 { return 0.0; }

    10.0 * (num_mag2 / den_mag2).log10()
}

/// Convert a stack of parametric filters into 31 graphic EQ band values (dB)
pub fn parametric_to_graphic(filters: &[EqFilter], preamp_db: f64) -> [f64; 31] {
    let mut bands = [0f64; 31];
    for (i, &freq) in ISO_31_BANDS.iter().enumerate() {
        let total_gain: f64 = filters.iter()
            .map(|f| biquad_gain_at_freq(f, freq))
            .sum();
        // Clamp to FxSound's ±12dB range, add preamp offset
        bands[i] = (total_gain + preamp_db).clamp(-12.0, 12.0);
    }
    bands
}

/// Write the 31-band EQ values to FxSound's registry and signal it to reload
pub fn apply_to_fxsound(bands: &[f64; 31], enabled: bool, volume_db: f64) -> Result<(), String> {
    // Build a PowerShell script that writes all 31 bands + master gain in one shot
    let bands_json: Vec<String> = bands.iter().enumerate()
        .map(|(i, &v)| format!("Set-ItemProperty -Path $eqPath -Name 'Band{}' -Value '{:.4}' -Type String", i+1, v))
        .collect();

    let ps_script = format!(r#"
$basePath = 'HKCU:\Software\FxSound\FxSound\11\1\LastUsed'
$eqPath = "$basePath\EQ"

# Create EQ key if it doesn't exist
if (-not (Test-Path $eqPath)) {{ New-Item -Path $eqPath -Force | Out-Null }}

# Write EQ on/off
Set-ItemProperty -Path $eqPath -Name 'EQOn' -Value '{eq_on}' -Type String

# Write all 31 band values
{bands}

# Write master gain (volume boost)
$masterPath = "$basePath\EQ"
Set-ItemProperty -Path $masterPath -Name 'MasterGain' -Value '{master:.4}' -Type String

Write-Output 'OK'
"#,
        eq_on = if enabled { "1" } else { "0" },
        bands = bands_json.join("\n"),
        master = volume_db,
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_script])
        .output()
        .map_err(|e| format!("PowerShell failed: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim() == "OK" {
        log::info!("Auris: EQ written to FxSound registry ({} bands, enabled={})", bands.len(), enabled);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Registry write failed: {}", stderr.trim()))
    }
}

/// Full pipeline: take Auris parametric filters → convert → write to FxSound
pub fn push_profile_to_fxsound(
    filters: &[EqFilter],
    preamp_db: f64,
    enabled: bool,
    volume_db: f64,
) -> Result<(), String> {
    if enabled {
        let bands = parametric_to_graphic(filters, preamp_db);
        apply_to_fxsound(&bands, true, volume_db)
    } else {
        // Flat EQ when disabled
        apply_to_fxsound(&[0.0f64; 31], false, 0.0)
    }
}

/// Detect the actual FxSound registry path on this machine
pub fn detect_fxsound_registry_path() -> Option<String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            "Get-ChildItem 'HKCU:\\Software\\FxSound' -Recurse -ErrorAction SilentlyContinue | Select-Object -ExpandProperty PSPath | Out-String"
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.contains("FxSound") {
        Some(stdout.trim().to_string())
    } else {
        None
    }
}
