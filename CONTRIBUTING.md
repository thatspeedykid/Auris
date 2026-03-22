# Contributing to Auris

Thank you for your interest in contributing. Auris is an open source project licensed under AGPL v3.0 and welcomes contributions of all kinds.

---

## Before you start

- Check [existing issues](https://github.com/privacychase/auris/issues) before opening a new one
- For large features, open a Discussion first so we can align before you invest time building
- All contributions must be compatible with AGPL v3.0 — do not introduce dependencies with incompatible licenses (MIT, Apache 2.0, and LGPL are fine; GPL v2-only is not)

---

## Types of contributions

### Bug reports
Open a GitHub Issue with:
- OS version and build number
- Auris version
- Steps to reproduce
- Expected vs actual behaviour
- Relevant logs from the privacy audit log if applicable

### Headphone profiles
The most accessible way to contribute. If you own a pair of headphones not in the AutoEQ database, you can submit measured correction curves. See `/headphones/README.md` for the format.

### Code contributions
1. Fork the repo
2. Create a branch from `dev`: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Ensure `npm run tauri build` completes without errors
5. Open a PR against the `dev` branch (not `main`)

### UI/design
We are actively looking for design contributions during Phase 1. Open a Discussion with mockups or design proposals.

---

## Code style

- Rust: `cargo fmt` and `cargo clippy` must pass
- TypeScript: ESLint config is included, run `npm run lint`
- C/C++ (DSP): follow the existing style in `/dsp` — we are not modifying this layer in v1.0

---

## Privacy rule

No contribution may add network calls, telemetry, analytics, or any form of data collection. This is a hard rule with no exceptions. PRs that add these will be closed immediately.

---

## Commit messages

Follow conventional commits:
```
feat: add headphone auto-detection on bluetooth connect
fix: correct EQ curve not applying on app switch
chore: update AutoEQ database to latest
docs: update build instructions for VS 2022
```

---

## License

By contributing, you agree that your contributions will be licensed under AGPL v3.0.
