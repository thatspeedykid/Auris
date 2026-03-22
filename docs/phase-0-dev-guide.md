# Phase 0 — Getting audio passthrough working

This is the first engineering milestone. Goal: confirm the FxSound DSP engine
processes audio through the new Tauri shell end-to-end.

---

## Prerequisites

Install these before anything else:

| Tool | Version | Link |
|------|---------|------|
| Rust | stable | https://rustup.rs |
| Node.js | 20+ | https://nodejs.org |
| Visual Studio 2022 | with C++ workload | https://visualstudio.microsoft.com |
| Windows SDK | latest | included in VS installer |
| FxSound (for the driver) | latest | https://github.com/fxsound2/fxsound-app/releases |

> **Driver note:** Install FxSound once to get the signed virtual audio driver,
> then uninstall the FxSound app. The driver stays installed. Auris uses it.
> We will ship a standalone driver installer in v1.0.

---

## Step 1 — Clone and install

```bash
git clone https://github.com/thatspeedykid/Auris.git
cd auris
npm install
```

---

## Step 2 — Run the dev shell

```bash
npm run tauri dev
```

This opens the Tauri window with the React UI. At this stage it's just a
placeholder — no audio processing yet. Confirm the window opens and the
UI renders correctly.

---

## Step 3 — Wire the DSP (the actual Phase 0 work)

The `/dsp` directory contains the FxSound C/C++ DSP engine. The task is to:

1. Compile `DfxDsp` as a static library from `/dsp`
2. Create a Rust FFI binding in `/src-tauri/src/dsp.rs` that calls into it
3. Hook the Tauri backend to route Windows audio through the DSP
4. Expose a Tauri command `get_dsp_status` that the React UI can call

The FxSound original build instructions are in the upstream repo:
https://github.com/fxsound2/fxsound-app#build-instructions

For the FFI bridge, use the `cc` crate in Cargo.toml to compile the C++ source,
and `bindgen` to generate Rust bindings from the headers.

---

## Step 4 — Confirm passthrough

When DSP is wired:
- Play audio on your system
- The status badge in the UI should show "DSP active"
- Audio should pass through cleanly with no artifacts
- Open Task Manager — Auris should use <50MB RAM at idle

Phase 0 is complete when audio passthrough works and the status badge is live.

---

## Troubleshooting

**"DSP not connected" badge after wiring**
- Check that the FxSound driver is installed: Device Manager → Sound → look for "FxSound Audio Enhancer"
- Check the Tauri console for errors: `npm run tauri dev` prints Rust panics to stdout

**Build fails on C++ compilation**
- Make sure Visual Studio C++ workload is installed
- Run from a "Developer Command Prompt for VS 2022"

**Audio crackles or drops**
- Check the buffer size in the DSP config
- Disable Windows audio enhancements on the output device
