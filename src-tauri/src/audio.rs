// Auris — WASAPI audio engine (Phase 2)
// Uses cpal to open the default output device and process every sample
// through the EQ chain + de-esser in real time.

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crate::eq::{EqChain, Desser, HeadphoneProfile};
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

pub fn start_engine(
    profile: HeadphoneProfile,
    shared: SharedState,
) -> Result<(), String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::Mutex;

    let host = cpal::default_host();

    let output_device = host.default_output_device()
        .ok_or("No output device found")?;

    let device_name = output_device.name().unwrap_or_default();
    log::info!("Auris audio engine: output device = {}", device_name);

    let supported_config = output_device
        .default_output_config()
        .map_err(|e| format!("Failed to get output config: {e}"))?;

    let sample_rate = supported_config.sample_rate().0 as f64;
    let channels = supported_config.channels() as usize;

    log::info!("Auris: sample_rate={} channels={}", sample_rate, channels);

    let eq_chain = Arc::new(Mutex::new(
        EqChain::new(profile.filters.clone(), profile.preamp_db, sample_rate)
    ));
    let desser = Arc::new(Mutex::new(Desser::new(sample_rate)));

    let shared_eq = shared.clone();
    let eq_chain_cb = eq_chain.clone();
    let desser_cb = desser.clone();

    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => {
            output_device.build_output_stream(
                &supported_config.into(),
                move |data: &mut [f32], _| {
                    if !shared_eq.eq_enabled.load(Ordering::Relaxed) {
                        return;
                    }
                    let mut eq = eq_chain_cb.lock().unwrap();
                    let mut ds = desser_cb.lock().unwrap();
                    let mut i = 0;
                    while i + 1 < data.len() {
                        let (l, r) = eq.process_stereo(data[i], data[i + 1]);
                        let (l, r) = if shared_eq.desser_enabled.load(Ordering::Relaxed) {
                            ds.process_stereo(l, r)
                        } else {
                            (l, r)
                        };
                        data[i]     = l;
                        data[i + 1] = r;
                        i += channels;
                    }
                },
                |err| log::error!("Auris stream error: {}", err),
                None,
            ).map_err(|e| format!("Failed to build output stream: {e}"))?
        }
        fmt => return Err(format!("Unsupported sample format: {:?}", fmt)),
    };

    stream.play().map_err(|e| format!("Failed to start stream: {e}"))?;

    shared.running.store(true, Ordering::Relaxed);

    // Leak the stream intentionally — it lives for the app lifetime.
    // This avoids the !Send issue with spawning threads.
    // On app exit, the OS cleans up the audio device handle.
    std::mem::forget(stream);

    log::info!("Auris audio engine running — EQ active on {}", device_name);
    Ok(())
}

pub fn stop_engine(shared: &SharedState) {
    // Signal stopped — stream is leaked so we just flip the flag.
    // Full stop/restart will be wired in Phase 3.
    shared.running.store(false, Ordering::Relaxed);
    log::info!("Auris audio engine: stop signalled");
}
