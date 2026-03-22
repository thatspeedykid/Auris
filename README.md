# Auris — by PrivacyChase

> **Your headphones, everywhere. Your audio, yours alone.**

Auris is a privacy-first, AI-enhanced audio enhancement application. It makes your headphones sound as good on Windows as they do on the manufacturer's app — with zero telemetry, zero cloud, and zero compromise.

Built as a fork of [FxSound](https://github.com/fxsound2/fxsound-app) (AGPL v3), Auris replaces the original GUI entirely and adds a modern feature set on top of a proven DSP engine.

---

## What Auris does

- **Headphone profiles** — auto-detects your connected headphones and applies a correction curve from the [AutoEQ](https://github.com/jaakkopasanen/AutoEq) database (1,400+ devices). AirPods on Windows. Sony XM5 on Linux. Beats on anything. All sounding the way they were meant to.
- **Per-app audio profiles** — Spotify gets one EQ curve. Discord gets voice clarity mode. Your game gets its own profile. Switches automatically.
- **On-device noise suppression** — deep mic noise removal via DeepFilterNet 3. No cloud processing. Runs entirely on your CPU.
- **AI scene detection** — detects whether you're listening to music, on a call, gaming, or watching video and applies the right preset automatically.
- **Zero telemetry** — no analytics, no crash reporting, no usage data. Nothing leaves your machine. Ever. There is a built-in privacy audit log that should always be empty — and if it isn't, that's a bug.

---

## Current status

> **Alpha — v0.1.0 (in development)**
> This project is in early development. Nothing is ready for daily use yet. We are in Phase 0: wiring the FxSound DSP engine to a new Tauri shell and confirming audio passthrough works end-to-end.

| Phase | Status | Description |
|-------|--------|-------------|
| 0 — Foundation | 🔧 In progress | Fork DSP, scaffold Tauri shell, confirm audio passthrough |
| 1 — Core shell | ⏳ Planned | Full React UI, EQ visualizer, presets, system tray |
| 2 — Per-app profiles | ⏳ Planned | WASAPI session monitoring, auto-switch |
| 3 — Noise suppression | ⏳ Planned | DeepFilterNet 3 via ONNX Runtime |
| 4 — AI scene detection | ⏳ Planned | On-device ONNX classifier, auto-preset |
| 5 — Headphone profiles | ⏳ Planned | AutoEQ integration, 1,400+ device database |
| 6 — Polish & release | ⏳ Planned | Signing, installer, PrivacyChase.com listing |

---

## Platform roadmap

| Platform | Version | Notes |
|----------|---------|-------|
| **Windows 10/11** | v1.0 | Primary target |
| macOS | v2.0 | Planned |
| Linux | v2.0 | Planned |
| Android | v3.0 | Planned |
| iOS | v3.0 | Planned |

The long-term vision: AirPods sounding great on Android. Sony XM5 sounding great on iPhone. Beats working properly on Linux. One app, every platform, every headphone.

---

## Tech stack

| Layer | Technology |
|-------|-----------|
| Audio DSP | C/C++ — forked from FxSound |
| App shell | [Tauri v2](https://tauri.app) (Rust) |
| UI | React 19 + TypeScript |
| AI inference | ONNX Runtime (CPU, on-device) |
| Noise suppression | DeepFilterNet 3 |
| Headphone EQ | AutoEQ database (MIT) |
| Per-app detection | Windows Audio Session API (WASAPI) |
| Build/release | GitHub Actions |

---

## Privacy charter

These are binding commitments for every release:

1. Zero network calls during normal operation.
2. The only permitted outbound connection is an optional GitHub releases check — user-initiated, off by default.
3. No analytics, crash reporting, or usage telemetry of any kind.
4. All user data (presets, profiles, settings) stored locally only.
5. A built-in privacy audit log records every system call. It should always be empty.
6. Source code is fully open (AGPL v3) so anyone can verify these commitments.

---

## Building from source

> Prerequisites: [Rust stable](https://rustup.rs), [Node.js 20+](https://nodejs.org), [Visual Studio 2022](https://visualstudio.microsoft.com), Windows SDK

```bash
# Clone the repo
git clone https://github.com/thatspeedykid/Auris.git
cd auris

# Install JS dependencies
npm install

# Run in development mode
npm run tauri dev

# Build a release binary
npm run tauri build
```

> **Note:** Auris requires the FxSound virtual audio driver to be installed for audio processing to work. You can install the driver by downloading the latest [FxSound release](https://github.com/fxsound2/fxsound-app/releases) and running it once — then uninstall the FxSound app and keep the driver. We are working on a standalone driver installer for v1.0.

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

- **Bug reports** → [GitHub Issues](https://github.com/thatspeedykid/Auris/issues)
- **Feature requests** → [GitHub Discussions](https://github.com/thatspeedykid/Auris/discussions)
- **Security issues** → see [SECURITY.md](SECURITY.md)

---

## Acknowledgements

Auris is built on the shoulders of:

- [FxSound](https://github.com/fxsound2/fxsound-app) — the DSP engine and audio driver (AGPL v3)
- [AutoEQ](https://github.com/jaakkopasanen/AutoEq) — headphone measurement database (MIT)
- [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet) — neural noise suppression (MIT)
- [Tauri](https://tauri.app) — the app shell framework (MIT/Apache 2.0)
- [JUCE](https://juce.com) — not used in Auris, but the original FxSound GUI was built with it

---

## License

Auris is licensed under the [GNU Affero General Public License v3.0](LICENSE).

This project is a fork of [FxSound](https://github.com/fxsound2/fxsound-app). Original FxSound copyright notices are preserved in the `/dsp` and `/audiopassthru` directories as required by the AGPL v3 license. See [CHANGES.md](CHANGES.md) for a full list of modifications made from the upstream source.

---

<p align="center">
  <sub>Auris — by <a href="https://privacychase.com">PrivacyChase</a> &nbsp;|&nbsp; Built by PrivacyChase — software that respects you.</sub>
</p>
