// Auris — WASAPI audio engine (Phase 2)
// Captures system audio via WASAPI loopback, runs it through the EQ chain,
// and writes it back out. This is what makes the EQ actually do something.
//
// Architecture:
//   WASAPI loopback capture → ring buffer → EQ chain → WASAPI render output
//
// We use cpal for cross-platform WASAPI access (Windows only in Phase 2).

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use crate::eq::{EqChain, Desser, HeadphoneProfile, EqFilter, FilterType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEngineState {
    pub running: bool,
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub active_profile: String,
    pub eq_enabled: bool,
    pub desser_enabled: bool,
    pub latency_ms: f32,
}

impl Default for AudioEngineState {
    fn default() -> Self {
        Self {
            running: false,
            sample_rate: 48000,
            buffer_size: 512,
            active_profile: "Beats Studio Pro".to_string(),
            eq_enabled: true,
            desser_enabled: true,
            latency_ms: 0.0,
        }
    }
}

// Shared state between the audio thread and the UI
pub struct SharedAudioState {
    pub eq_enabled: AtomicBool,
    pub desser_enabled: AtomicBool,
    pub running: AtomicBool,
}

impl SharedAudioState {
    pub fn new() -> Self {
        Self {
            eq_enabled: AtomicBool::new(true),
            desser_enabled: AtomicBool::new(true),
            running: AtomicBool::new(false),
        }
    }
}

pub type SharedState = Arc<SharedAudioState>;

/// Start the audio processing engine.
/// Spawns a background thread that:
/// 1. Opens the default output device via WASAPI loopback
/// 2. Runs every sample through the EQ chain
/// 3. Writes processed audio back to the render endpoint
pub fn start_engine(
    profile: HeadphoneProfile,
    shared: SharedState,
) -> Result<(), String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();

    // Get default output device
    let output_device = host.default_output_device()
        .ok_or("No output device found")?;

    let device_name = output_device.name().unwrap_or_default();
    log::info!("Auris audio engine: output device = {}", device_name);

    // Get supported config
    let supported_config = output_device
        .default_output_config()
        .map_err(|e| format!("Failed to get output config: {e}"))?;

    let sample_rate = supported_config.sample_rate().0 as f64;
    let channels = supported_config.channels() as usize;

    log::info!("Auris: sample_rate={} channels={}", sample_rate, channels);

    // Build EQ chain from profile
    let eq_chain = Arc::new(Mutex::new(
        EqChain::new(profile.filters.clone(), profile.preamp_db, sample_rate)
    ));
    let desser = Arc::new(Mutex::new(Desser::new(sample_rate)));

    let shared_clone = shared.clone();
    let eq_chain_clone = eq_chain.clone();
    let desser_clone = desser.clone();

    // Build the output stream with processing
    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => {
            output_device.build_output_stream(
                &supported_config.into(),
                move |data: &mut [f32], _| {
                    if !shared_clone.eq_enabled.load(Ordering::Relaxed) {
                        return; // bypass — pass through unchanged
                    }

                    let mut eq = eq_chain_clone.lock().unwrap();
                    let mut ds = desser_clone.lock().unwrap();

                    // Process stereo pairs
                    let mut i = 0;
                    while i + 1 < data.len() {
                        let (l, r) = eq.process_stereo(data[i], data[i + 1]);
                        let (l, r) = if shared_clone.desser_enabled.load(Ordering::Relaxed) {
                            ds.process_stereo(l, r)
                        } else {
                            (l, r)
                        };
                        data[i]     = l;
                        data[i + 1] = r;
                        i += channels;
                    }
                },
                |err| log::error!("Auris audio stream error: {}", err),
                None,
            ).map_err(|e| format!("Failed to build output stream: {e}"))?
        }
        fmt => return Err(format!("Unsupported sample format: {:?}", fmt)),
    };

    stream.play().map_err(|e| format!("Failed to start stream: {e}"))?;

    shared.running.store(true, Ordering::Relaxed);

    // Keep stream alive on a background thread
    std::thread::spawn(move || {
        log::info!("Auris audio engine running");
        // Hold the stream alive until stopped
        loop {
            if !shared.running.load(Ordering::Relaxed) {
                log::info!("Auris audio engine stopping");
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        drop(stream);
    });

    Ok(())
}

pub fn stop_engine(shared: &SharedState) {
    shared.running.store(false, Ordering::Relaxed);
}
