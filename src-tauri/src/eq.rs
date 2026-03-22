// Auris — EQ engine
// Biquad parametric EQ implementation in pure Rust
// Each filter is a biquad IIR filter (cookbook formulas by Robert Bristow-Johnson)

use std::f64::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    Peaking,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqFilter {
    pub filter_type: FilterType,
    pub fc: f64,    // center frequency in Hz
    pub gain: f64,  // gain in dB
    pub q: f64,     // Q factor
    pub enabled: bool,
}

// Biquad coefficients
#[derive(Debug, Clone)]
struct BiquadCoeffs {
    b0: f64, b1: f64, b2: f64,
    a1: f64, a2: f64,
}

// Per-channel biquad state
#[derive(Debug, Clone, Default)]
struct BiquadState {
    x1: f64, x2: f64,
    y1: f64, y2: f64,
}

fn compute_coeffs(filter: &EqFilter, sample_rate: f64) -> BiquadCoeffs {
    let w0 = 2.0 * PI * filter.fc / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 / (2.0 * filter.q);
    let a = 10f64.powf(filter.gain / 40.0); // for peaking: sqrt(10^(dB/20))

    match filter.filter_type {
        FilterType::Peaking => {
            let b0 = 1.0 + alpha * a;
            let b1 = -2.0 * cos_w0;
            let b2 = 1.0 - alpha * a;
            let a0 = 1.0 + alpha / a;
            let a1 = -2.0 * cos_w0;
            let a2 = 1.0 - alpha / a;
            BiquadCoeffs { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 }
        }
        FilterType::LowShelf => {
            let b0 = a * ((a+1.0) - (a-1.0)*cos_w0 + 2.0*a.sqrt()*alpha);
            let b1 = 2.0 * a * ((a-1.0) - (a+1.0)*cos_w0);
            let b2 = a * ((a+1.0) - (a-1.0)*cos_w0 - 2.0*a.sqrt()*alpha);
            let a0 = (a+1.0) + (a-1.0)*cos_w0 + 2.0*a.sqrt()*alpha;
            let a1 = -2.0 * ((a-1.0) + (a+1.0)*cos_w0);
            let a2 = (a+1.0) + (a-1.0)*cos_w0 - 2.0*a.sqrt()*alpha;
            BiquadCoeffs { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 }
        }
        FilterType::HighShelf => {
            let b0 = a * ((a+1.0) + (a-1.0)*cos_w0 + 2.0*a.sqrt()*alpha);
            let b1 = -2.0 * a * ((a-1.0) + (a+1.0)*cos_w0);
            let b2 = a * ((a+1.0) + (a-1.0)*cos_w0 - 2.0*a.sqrt()*alpha);
            let a0 = (a+1.0) - (a-1.0)*cos_w0 + 2.0*a.sqrt()*alpha;
            let a1 = 2.0 * ((a-1.0) - (a+1.0)*cos_w0);
            let a2 = (a+1.0) - (a-1.0)*cos_w0 - 2.0*a.sqrt()*alpha;
            BiquadCoeffs { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 }
        }
        FilterType::LowPass => {
            let b0 = (1.0 - cos_w0) / 2.0;
            let b1 = 1.0 - cos_w0;
            let b2 = (1.0 - cos_w0) / 2.0;
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * cos_w0;
            let a2 = 1.0 - alpha;
            BiquadCoeffs { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 }
        }
        FilterType::HighPass => {
            let b0 = (1.0 + cos_w0) / 2.0;
            let b1 = -(1.0 + cos_w0);
            let b2 = (1.0 + cos_w0) / 2.0;
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * cos_w0;
            let a2 = 1.0 - alpha;
            BiquadCoeffs { b0: b0/a0, b1: b1/a0, b2: b2/a0, a1: a1/a0, a2: a2/a0 }
        }
    }
}

fn process_sample(sample: f64, coeffs: &BiquadCoeffs, state: &mut BiquadState) -> f64 {
    let y = coeffs.b0 * sample + coeffs.b1 * state.x1 + coeffs.b2 * state.x2
          - coeffs.a1 * state.y1 - coeffs.a2 * state.y2;
    state.x2 = state.x1;
    state.x1 = sample;
    state.y2 = state.y1;
    state.y1 = y;
    y
}

// The full EQ chain — a stack of biquad filters applied in series
pub struct EqChain {
    filters: Vec<EqFilter>,
    preamp_db: f64,
    sample_rate: f64,
    coeffs: Vec<BiquadCoeffs>,
    states_l: Vec<BiquadState>,
    states_r: Vec<BiquadState>,
}

impl EqChain {
    pub fn new(filters: Vec<EqFilter>, preamp_db: f64, sample_rate: f64) -> Self {
        let coeffs: Vec<BiquadCoeffs> = filters.iter()
            .map(|f| compute_coeffs(f, sample_rate))
            .collect();
        let n = filters.len();
        Self {
            filters,
            preamp_db,
            sample_rate,
            coeffs,
            states_l: vec![BiquadState::default(); n],
            states_r: vec![BiquadState::default(); n],
        }
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let preamp_linear = 10f64.powf(self.preamp_db / 20.0);
        let mut l = left as f64 * preamp_linear;
        let mut r = right as f64 * preamp_linear;

        for (i, filter) in self.filters.iter().enumerate() {
            if filter.enabled {
                l = process_sample(l, &self.coeffs[i], &mut self.states_l[i]);
                r = process_sample(r, &self.coeffs[i], &mut self.states_r[i]);
            }
        }

        (l as f32, r as f32)
    }

    pub fn update_filters(&mut self, filters: Vec<EqFilter>, preamp_db: f64) {
        let coeffs: Vec<BiquadCoeffs> = filters.iter()
            .map(|f| compute_coeffs(f, self.sample_rate))
            .collect();
        let n = filters.len();
        self.filters = filters;
        self.preamp_db = preamp_db;
        self.coeffs = coeffs;
        self.states_l = vec![BiquadState::default(); n];
        self.states_r = vec![BiquadState::default(); n];
    }
}

// De-esser: frequency-specific compressor targeting sibilance (5-10 kHz)
pub struct Desser {
    pub enabled: bool,
    pub threshold_db: f64,   // default: -18 dB
    pub ratio: f64,           // default: 4.0
    pub frequency: f64,       // sibilance center: 7500 Hz
    pub bandwidth_q: f64,     // Q for detection band
    sample_rate: f64,
    detect_coeffs: BiquadCoeffs,
    detect_state_l: BiquadState,
    detect_state_r: BiquadState,
    cut_coeffs: BiquadCoeffs,
    cut_state_l: BiquadState,
    cut_state_r: BiquadState,
    // Envelope follower state
    envelope_l: f64,
    envelope_r: f64,
    attack_coeff: f64,
    release_coeff: f64,
}

impl Desser {
    pub fn new(sample_rate: f64) -> Self {
        let freq = 7500.0;
        let q = 2.0;
        let detect_filter = EqFilter {
            filter_type: FilterType::Peaking,
            fc: freq,
            gain: 12.0, // boost detection band so envelope follower is sensitive
            q,
            enabled: true,
        };
        let cut_filter = EqFilter {
            filter_type: FilterType::Peaking,
            fc: freq,
            gain: 0.0, // gain set dynamically
            q,
            enabled: true,
        };
        // Attack 1ms, release 60ms
        let attack_coeff = (-1.0 / (0.001 * sample_rate)).exp();
        let release_coeff = (-1.0 / (0.060 * sample_rate)).exp();

        Self {
            enabled: true,
            threshold_db: -18.0,
            ratio: 4.0,
            frequency: freq,
            bandwidth_q: q,
            sample_rate,
            detect_coeffs: compute_coeffs(&detect_filter, sample_rate),
            detect_state_l: BiquadState::default(),
            detect_state_r: BiquadState::default(),
            cut_coeffs: compute_coeffs(&cut_filter, sample_rate),
            cut_state_l: BiquadState::default(),
            cut_state_r: BiquadState::default(),
            envelope_l: 0.0,
            envelope_r: 0.0,
            attack_coeff,
            release_coeff,
        }
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        if !self.enabled {
            return (left, right);
        }

        let l = left as f64;
        let r = right as f64;

        // Detect sibilance energy
        let detect_l = process_sample(l, &self.detect_coeffs, &mut self.detect_state_l).abs();
        let detect_r = process_sample(r, &self.detect_coeffs, &mut self.detect_state_r).abs();

        // Envelope follower
        let coeff_l = if detect_l > self.envelope_l { self.attack_coeff } else { self.release_coeff };
        let coeff_r = if detect_r > self.envelope_r { self.attack_coeff } else { self.release_coeff };
        self.envelope_l = detect_l + coeff_l * (self.envelope_l - detect_l);
        self.envelope_r = detect_r + coeff_r * (self.envelope_r - detect_r);

        // Compute gain reduction
        let threshold_linear = 10f64.powf(self.threshold_db / 20.0);
        let gain_l = if self.envelope_l > threshold_linear {
            let excess_db = 20.0 * (self.envelope_l / threshold_linear).log10();
            let reduction_db = excess_db * (1.0 - 1.0 / self.ratio);
            10f64.powf(-reduction_db / 20.0)
        } else { 1.0 };
        let gain_r = if self.envelope_r > threshold_linear {
            let excess_db = 20.0 * (self.envelope_r / threshold_linear).log10();
            let reduction_db = excess_db * (1.0 - 1.0 / self.ratio);
            10f64.powf(-reduction_db / 20.0)
        } else { 1.0 };

        ((l * gain_l) as f32, (r * gain_r) as f32)
    }
}

// Built-in headphone profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadphoneProfile {
    pub name: String,
    pub device_match: Vec<String>, // WASAPI device name substrings to match
    pub preamp_db: f64,
    pub filters: Vec<EqFilter>,
}

pub fn beats_studio_pro_profile() -> HeadphoneProfile {
    // Measurements from Rtings, corrected to Harman over-ear 2018 target
    // Beats Studio Pro has a V-shaped signature: elevated bass and treble,
    // recessed mids around 1-2kHz, slight upper-treble glare around 8-10kHz
    HeadphoneProfile {
        name: "Beats Studio Pro".to_string(),
        device_match: vec![
            "Beats Studio Pro".to_string(),
            "Beats Studio".to_string(),
        ],
        preamp_db: -6.5,
        filters: vec![
            // Cut the bass shelf — Beats over-emphasizes sub-bass
            EqFilter { filter_type: FilterType::LowShelf,  fc: 105.0,  gain: -4.5, q: 0.70, enabled: true },
            // Small cut around 200Hz where it gets muddy
            EqFilter { filter_type: FilterType::Peaking,   fc: 210.0,  gain: -2.0, q: 0.80, enabled: true },
            // Lift the recessed upper-mids — vocals and guitar clarity
            EqFilter { filter_type: FilterType::Peaking,   fc: 1200.0, gain:  3.5, q: 1.20, enabled: true },
            // Presence lift — adds definition to voices
            EqFilter { filter_type: FilterType::Peaking,   fc: 3200.0, gain:  2.5, q: 1.50, enabled: true },
            // Tame the treble glare that causes fatigue on long sessions
            EqFilter { filter_type: FilterType::Peaking,   fc: 6300.0, gain: -3.0, q: 2.00, enabled: true },
            // Cut harsh upper treble — this is where the "shh" sibilance lives
            EqFilter { filter_type: FilterType::Peaking,   fc: 9000.0, gain: -2.5, q: 1.80, enabled: true },
            // Air shelf — gentle top-end sparkle restoration
            EqFilter { filter_type: FilterType::HighShelf, fc: 12000.0, gain: 1.5, q: 0.70, enabled: true },
        ],
    }
}

pub fn built_in_profiles() -> Vec<HeadphoneProfile> {
    vec![
        beats_studio_pro_profile(),
        sony_xm5_profile(),
        airpods_pro_profile(),
        flat_profile(),
    ]
}

fn sony_xm5_profile() -> HeadphoneProfile {
    HeadphoneProfile {
        name: "Sony WH-1000XM5".to_string(),
        device_match: vec!["WH-1000XM5".to_string(), "Sony WH".to_string()],
        preamp_db: -5.5,
        filters: vec![
            EqFilter { filter_type: FilterType::LowShelf,  fc: 105.0,  gain: -3.5, q: 0.70, enabled: true },
            EqFilter { filter_type: FilterType::Peaking,   fc: 900.0,  gain:  2.0, q: 1.40, enabled: true },
            EqFilter { filter_type: FilterType::Peaking,   fc: 3500.0, gain:  3.0, q: 1.60, enabled: true },
            EqFilter { filter_type: FilterType::Peaking,   fc: 7000.0, gain: -4.0, q: 2.20, enabled: true },
            EqFilter { filter_type: FilterType::HighShelf, fc: 10000.0, gain: 2.0, q: 0.70, enabled: true },
        ],
    }
}

fn airpods_pro_profile() -> HeadphoneProfile {
    HeadphoneProfile {
        name: "Apple AirPods Pro".to_string(),
        device_match: vec!["AirPods Pro".to_string(), "AirPods".to_string()],
        preamp_db: -4.0,
        filters: vec![
            EqFilter { filter_type: FilterType::Peaking,   fc: 80.0,   gain: -2.0, q: 0.80, enabled: true },
            EqFilter { filter_type: FilterType::Peaking,   fc: 2800.0, gain:  2.5, q: 1.20, enabled: true },
            EqFilter { filter_type: FilterType::Peaking,   fc: 6000.0, gain: -3.5, q: 2.00, enabled: true },
            EqFilter { filter_type: FilterType::HighShelf, fc: 11000.0, gain: 1.0, q: 0.70, enabled: true },
        ],
    }
}

fn flat_profile() -> HeadphoneProfile {
    HeadphoneProfile {
        name: "Flat (No EQ)".to_string(),
        device_match: vec![],
        preamp_db: 0.0,
        filters: vec![],
    }
}
