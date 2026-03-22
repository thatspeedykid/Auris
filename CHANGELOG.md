# Changelog

All notable changes to Auris will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Auris uses [semantic versioning](https://semver.org): `MAJOR.MINOR.PATCH`.
Pre-release builds use suffixes: `v0.1.0-alpha.1`, `v0.2.0-beta.1`.

---

## [Unreleased]

### In progress
- Phase 0: Tauri shell scaffolding
- Phase 0: FxSound DSP wired to new Rust backend
- Phase 0: Audio passthrough confirmed working

---

## [0.1.0-alpha] — upcoming

### Added
- Initial project structure
- FxSound DSP engine (forked from v1.2.6.0, unmodified)
- Tauri v2 application shell (Rust backend + React frontend)
- GitHub Actions build and release pipeline
- Privacy audit log foundation
- Repository documentation (README, CONTRIBUTING, SECURITY, CHANGES)

### Notes
- No UI beyond a placeholder window
- Audio passthrough working but no DSP controls exposed yet
- Not suitable for daily use

---

_Auris is a fork of [FxSound](https://github.com/fxsound2/fxsound-app) (AGPL v3).  
Original FxSound changelog: see upstream repo._
