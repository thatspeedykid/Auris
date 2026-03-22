# Auris — Technical Architecture

## Overview

Auris is a desktop audio enhancement application built on three layers:

```
┌─────────────────────────────────────┐
│          React UI (TypeScript)       │  ← What the user sees
├─────────────────────────────────────┤
│        Tauri Shell (Rust)            │  ← App logic, OS integration
├─────────────────────────────────────┤
│     FxSound DSP Engine (C/C++)       │  ← Audio processing core
└─────────────────────────────────────┘
         ↓ Windows Audio Driver ↓
```

---

## Layer 1 — DSP Engine (C/C++)

**Source:** Forked from [FxSound](https://github.com/fxsound2/fxsound-app) v1.2.6.0  
**License:** AGPL v3  
**Location:** `/dsp`, `/audiopassthru`

The DSP engine installs as a virtual audio device on Windows.
All system audio routes through it before reaching the speakers/headphones.
Processing includes: EQ, bass boost, 3D surround, clarity, ambience.

In Auris v1.0, the DSP engine is used **unmodified**. We only replace
the control layer (what tells it what EQ curve to apply).

### Key files
- `dsp/` — DfxDsp module, the signal processing core
- `audiopassthru/` — bridges the DSP to Windows audio APIs
- `bin/` — signed Windows kernel driver (not modified, not rebuilt)

---

## Layer 2 — Tauri Backend (Rust)

**Framework:** [Tauri v2](https://tauri.app)  
**Location:** `/src-tauri`

The Rust backend handles:

- **DSP bridge** — calls into the FxSound DSP via FFI to apply EQ settings
- **WASAPI monitor** — watches Windows Audio Session API for active app sessions and triggers per-app profile switches
- **Device detector** — reads the connected audio device name and fuzzy-matches against the AutoEQ headphone database
- **ONNX inference** — runs scene detection and noise suppression models via ONNX Runtime
- **Profile store** — reads/writes user presets and per-app profiles to `%APPDATA%\PrivacyChase\Auris`
- **Privacy audit log** — records every system call; should always be empty of network activity
- **Tauri commands** — exposes all of the above to the React frontend via typed async commands

### Tauri commands (planned)
```rust
get_dsp_status()       -> DspStatus
set_eq_profile(profile: EqProfile)
get_connected_device() -> Option<DeviceInfo>
get_headphone_match()  -> Option<HeadphoneProfile>
list_app_sessions()    -> Vec<AppSession>
get_scene()            -> AudioScene
toggle_noise_suppression(enabled: bool)
get_privacy_log()      -> Vec<LogEntry>
```

---

## Layer 3 — React UI (TypeScript)

**Framework:** React 19 + TypeScript  
**Build tool:** Vite  
**Location:** `/src`

The UI communicates with the Rust backend exclusively via Tauri's
`invoke()` API. It never makes direct OS calls or network requests.

### Planned screens
- **Main panel** — EQ visualizer, preset selector, headphone profile status
- **Per-app profiles** — list of detected apps, assigned profiles, editor
- **Noise suppression** — mic selector, strength slider, live level meter
- **AI scene** — current scene, confidence, override, disable toggle
- **Privacy panel** — audit log viewer, zero-data confirmation, source link
- **System tray** — quick toggle, current preset, noise suppression on/off

---

## AI features

Both AI features run entirely on-device via ONNX Runtime.

### Scene detection
- Polls audio context every 5 seconds
- Small classifier model (~4MB ONNX)
- Output: Music | Speech | Gaming | Podcast | Silence
- Auto-applies matching preset; user can override or disable

### Noise suppression
- DeepFilterNet 3, exported to ONNX (~20MB CPU)
- Applied to mic input capture only — does not touch playback
- Configurable: subtle / moderate / aggressive
- Added latency: ~8ms at moderate settings

---

## Headphone profiles

AutoEQ correction profiles are stored as parametric EQ filter sets in `/headphones/`.
On device connect, the Rust backend:

1. Reads the WASAPI device name string
2. Fuzzy-matches against the device name index in `/headphones/`
3. Loads the matching `parametric_eq.txt` filter set
4. Passes the filters to the DSP engine

Users can also manually search and select from the full 1,400+ device library.

---

## Data storage

All user data is local. No cloud sync. No accounts.

```
%APPDATA%\PrivacyChase\Auris\
  config.json          — app settings
  profiles\            — named EQ presets
  app_profiles\        — per-app profile assignments
  privacy_log.json     — audit log (should always show 0 network calls)
```

---

## Build pipeline

See `.github/workflows/` for the full CI/CD setup.

| Trigger | Action |
|---------|--------|
| Push to `dev` | Build + lint check |
| Push to `main` | Build + artifact upload |
| Push tag `v*.*.*` | Full release build, sign, publish to GitHub Releases |

---

## Platform roadmap

| Platform | Notes |
|----------|-------|
| Windows 10/11 (x64) | v1.0 — primary target |
| macOS | v2.0 — CoreAudio replaces WASAPI layer |
| Linux | v2.0 — PipeWire/ALSA backend |
| Android | v3.0 — requires separate audio processing approach |
| iOS | v3.0 — most restricted platform, research needed |

The Tauri shell and React UI are already cross-platform.
The main porting work for v2.0 is replacing the Windows-specific
audio driver and WASAPI layer with platform equivalents.
