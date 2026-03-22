// DSP bridge — Phase 0
// This module will bridge to the FxSound C/C++ DSP engine via FFI.
// Currently a stub — returns placeholder state.

#[derive(Debug, PartialEq)]
pub enum DspStatus {
    Active,
    Inactive,
    Error(String),
}

pub fn get_status() -> String {
    // TODO Phase 0:
    // 1. Compile /dsp/DfxDsp as a static lib via build.rs
    // 2. Create bindgen bindings from DfxDsp headers
    // 3. Call into the DSP to check if the virtual audio driver is active
    // 4. Return actual status

    "inactive".to_string()
}

pub fn set_enabled(_enabled: bool) -> Result<(), String> {
    // TODO Phase 0: toggle DSP processing on/off
    Err("DSP not yet wired".to_string())
}
