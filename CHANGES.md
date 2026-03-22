# Changes from upstream (FxSound)

This file documents all significant changes made to the upstream FxSound source code
as required by the GNU Affero General Public License v3.0.

Upstream repository: https://github.com/fxsound2/fxsound-app  
Upstream license: AGPL v3.0  
Fork base: FxSound v1.2.6.0 (February 13, 2026)

---

## What was kept

- `/dsp` — DfxDsp module (C/C++ DSP processing engine), unmodified
- `/audiopassthru` — audio driver interaction layer, unmodified
- `/bin` — prebuilt signed Windows audio driver binaries, unmodified

Original copyright notices in all retained files are preserved as-is.

## What was removed

- `/fxsound` — the entire JUCE-based GUI application (replaced entirely)
- All JUCE framework dependencies
- All FxSound-specific branding, assets, and UI resources
- Telemetry and analytics hooks present in the original application

## What was added

- `/src-tauri` — new Rust/Tauri application shell (replaces JUCE GUI)
- `/src` — new React + TypeScript UI layer
- `/headphones` — AutoEQ headphone correction database (MIT license, separate project)
- `/models` — ONNX model files for on-device AI features
- `/.github/workflows` — GitHub Actions CI/CD pipeline
- New feature: per-application audio profiles via WASAPI
- New feature: on-device noise suppression (DeepFilterNet 3)
- New feature: AI scene detection (on-device ONNX classifier)
- New feature: headphone auto-detection and EQ correction (AutoEQ)
- New feature: privacy audit log

## Summary

The audio DSP core (the hard part) is reused unchanged. Everything a user sees and
interacts with is new. The product is fully open source under AGPL v3.0.
