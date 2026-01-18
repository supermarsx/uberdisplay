# UberDisplay

![license](https://img.shields.io/badge/license-MIT-green)
![status](https://img.shields.io/badge/status-spec%20%2B%20reference-blue)
![platform](https://img.shields.io/badge/platform-android%20%7C%20pc-lightgrey)

UberDisplay is a multi-component system that turns an Android device into an
external display and input surface for a host computer. This repository
includes the Android app (`android/`), SonarPen SDK samples
(`libs/sonarpen-sdk/`), and the authoritative product and protocol spec
(`spec.md`).

## Objectives
- Provide a low-latency Android mirror client with touch, pen, and keyboard
  input back to the host.
- Support USB (AOAP) and Wi-Fi transports using a stable, documented protocol.
- Maintain a clear separation between Android client, PC host app, and driver
  components as the project evolves.

## Supported / Target Features
Supported today (reference assets):
- Android app project under `android/`.
- SonarPen SDK and Android sample projects under `libs/sonarpen-sdk/`.
- Protocol and product specification in `spec.md`.

Targeted by the spec:
- Android client implementation (active in `android/`).
- PC host app (Tauri + Rust backend + Next.js frontend).
- Shared Rust protocol and transport crates.
- Windows virtual display driver integration.
- Enhanced input modes, action menu, and diagnostics.

## Where to Start
- Read `spec.md` for system goals, protocol details, and architecture targets.
- The Android app lives in `android/app/src/main/`.
