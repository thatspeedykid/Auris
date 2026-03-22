# ONNX Models

This directory contains on-device AI model files used by Auris.
All inference runs locally — nothing is sent to any server.

## Models

### scene_detector.onnx (planned — v1.0 Phase 4)
Classifies the current audio context every 5 seconds.
Output classes: Music | Speech/Call | Gaming | Podcast/Video | Silence
Size target: ~4MB. Trained on open audio datasets.

### noise_suppressor.onnx (planned — v1.0 Phase 3)
DeepFilterNet 3 — applied to mic input only.
Exported from https://github.com/Rikorose/DeepFilterNet (MIT license).
Size: ~20MB CPU-only ONNX export.

## Runtime

Models are run via ONNX Runtime for Windows (CPU).
GPU acceleration is optional and off by default.

## Placeholder

This directory is empty in v0.1.0-alpha.
Models will be added in Phase 3 and Phase 4 of development.
