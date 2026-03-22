// build.rs — Auris Tauri backend build script
//
// Phase 0: This will compile the FxSound DSP C/C++ engine as a static library.
// Currently a no-op placeholder.

fn main() {
    tauri_build::build();

    // TODO Phase 0: compile FxSound DSP
    // cc::Build::new()
    //     .cpp(true)
    //     .files(glob::glob("../dsp/src/**/*.cpp").unwrap().flatten())
    //     .include("../dsp/include")
    //     .flag_if_supported("/std:c++17")
    //     .compile("dfxdsp");
    //
    // println!("cargo:rustc-link-lib=static=dfxdsp");
    // println!("cargo:rerun-if-changed=../dsp/");
}
