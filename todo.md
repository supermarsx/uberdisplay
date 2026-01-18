# TODO

## Android Bootstrap (Kotlin)
- [x] Create `android/` project root and Gradle settings
- [x] Add root Gradle build files and properties
- [x] Create `app` module with Kotlin Android configuration
- [x] Add minimal `AndroidManifest.xml`
- [x] Add `MainActivity` and base layout/theme/resources
- [x] Add placeholder navigation/state UI for connection status
- [ ] Add Gradle wrapper for consistent builds
- [ ] Add Android `.gitignore` for build outputs

## Android Core Stubs
- [x] Define connection state enum
- [ ] Add transport interface stubs (USB AOAP, TCP)
- [ ] Add session controller skeleton (start/stop hooks)
- [ ] Add protocol constants scaffold
- [x] Add connection state binding to main UI
- [ ] Add TCP listener stub on port 1445
- [ ] Add AOAP attach listener stub

## Root Module Integration (Android)
- [x] Add root module toggle and status check in UI
- [x] Add socket handshake (HELLO/PING) stub
- [ ] Add settings screen entry for root module status
- [ ] Parse capability bitmask from root daemon response
- [ ] Add root status warning copy when unreachable

## Input + UI Stubs
- [ ] Touch input capture stub
- [ ] Pen/stylus input stub
- [ ] Keyboard input stub
- [ ] Action menu placeholder view
- [ ] Add fullscreen MirrorActivity placeholder

## SonarPen Integration (Scaffold)
- [ ] Add SDK module reference or dependency wiring
- [ ] Add microphone permission flow stub
- [ ] Add calibration activity placeholder
- [ ] Add SonarPen status indicator placeholder

## Root / Magisk Module
- [x] Add Magisk module skeleton under `magisk/uberdisplay-root/`
- [x] Define root IPC contract (socket path, version, capabilities bitmap)
- [x] Add stub root daemon entrypoint and health check response
- [x] Document install/uninstall flow for the Magisk module
- [ ] Add a `zip-module.ps1` helper script for packaging

## Testing
- [x] Add unit test source set and a basic test
- [x] Add instrumentation test skeleton
- [ ] Add a test for root socket handshake parsing
- [ ] Add a test for connection state transitions

## Docs
- [ ] Update `README.md` with Android build/run instructions
- [ ] Document root module IPC in `spec.md` or a linked appendix
