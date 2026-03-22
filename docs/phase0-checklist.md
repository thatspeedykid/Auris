# Phase 0 checklist — Foundation

This is the immediate to-do list before anything else.
Goal: audio flowing through the FxSound DSP engine, controlled by the new Tauri shell.
When this is done, Auris is a working (if featureless) audio processor.

---

## Setup

- [ ] Create GitHub repo `privacychase/auris` (public, AGPL v3)
- [ ] Push this scaffold as the first commit on `main`
- [ ] Create `dev` branch — all work happens here
- [ ] Confirm GitHub Actions `build.yml` passes on first push

## DSP wiring

- [ ] Copy `/dsp` and `/audiopassthru` from FxSound v1.2.6.0
- [ ] Preserve all original FxSound copyright headers
- [ ] Confirm DSP compiles with Visual Studio 2022 + Windows SDK
- [ ] Write a minimal Rust FFI wrapper in `/src-tauri/src/dsp.rs`
      that can call `DfxDsp` to enable/disable processing
- [ ] Install FxSound once (to get the signed audio driver), then uninstall app
- [ ] Confirm audio passthrough works: music plays through the DSP

## Tauri shell

- [ ] Run `npm create tauri-app` or use this scaffold directly
- [ ] Confirm `npm run tauri dev` opens a window on Windows
- [ ] Add a Tauri command `get_dsp_status()` wired to the DSP layer
- [ ] React UI reads DSP status and shows active/inactive badge

## Audio passthrough milestone

> **Definition of done for Phase 0:**
> Play music. Enable Auris. Hear it processed (even if with no custom EQ applied yet).
> The DSP engine is running and being controlled by the new shell.

## CI/CD

- [ ] Push to `dev` → `build.yml` compiles successfully
- [ ] Merge to `main` → build artifact appears in Actions
- [ ] Push tag `v0.1.0-alpha.1` → `release.yml` publishes a GitHub Release

---

## Phase 0 is NOT

- A working UI (placeholder is fine)
- Any EQ controls (coming in Phase 1)
- Any AI features (Phase 3 and 4)
- Any headphone detection (Phase 5)

Stay focused. Get audio flowing. Ship v0.1.0-alpha.1.
