# TODO

## Current Focus
- [x] Add protocol data type constants scaffold
- [x] Add basic transport manager to arbitrate TCP/AOAP
- [x] Add connection state LiveData/Flow for UI
- [x] Add connection lifecycle logging
- [x] Add error reporting stub hooks (no-op)
- [x] Parse capability bitmask from root daemon response
- [x] Add Gradle wrapper for consistent builds
- [x] Add transport lifecycle hooks to MirrorActivity
- [x] Add CI workflow for `gradle test`
- [x] Add basic lint config or suppressions
- [x] Document root module IPC in `spec.md` or a linked appendix
- [x] Add transport listener stub and hook it up
- [x] Add session manager stub and wire into connection controller
- [x] Add Action Menu activity placeholder
- [x] Add action menu model/config stubs
- [ ] Add action menu edit screen stub

## Android Bootstrap (Kotlin)
- [x] Create `android/` project root and Gradle settings
- [x] Add root Gradle build files and properties
- [x] Create `app` module with Kotlin Android configuration
- [x] Add minimal `AndroidManifest.xml`
- [x] Add `MainActivity` and base layout/theme/resources
- [x] Add placeholder navigation/state UI for connection status
- [x] Add Android `.gitignore` for build outputs

## Android Core Stubs
- [x] Define connection state enum
- [x] Add transport interface stubs (USB AOAP, TCP)
- [x] Add session controller skeleton (start/stop hooks)
- [x] Add protocol constants scaffold
- [x] Add connection state binding to main UI
- [x] Add TCP listener stub on port 1445
- [x] Add AOAP attach listener stub
- [x] Add protocol data type IDs
- [x] Add basic transport manager to arbitrate TCP/AOAP
- [x] Add connection lifecycle logging
- [x] Add connection state LiveData/Flow for UI
- [x] Add error reporting stub hooks (no-op)
- [x] Add transport listener stub
- [x] Add session manager stub

## Build & Tooling
- [x] Add Gradle wrapper for consistent builds
- [x] Add basic lint config or suppressions
- [x] Add CI workflow for `gradle test`

## Root Module Integration (Android)
- [x] Add root module toggle and status check in UI
- [x] Add socket handshake (HELLO/PING) stub
- [x] Add settings screen entry for root module status
- [x] Parse capability bitmask from root daemon response
- [x] Add root status warning copy when unreachable
- [x] Add root module toggle to settings screen
- [x] Add diagnostics toggle in settings

## Input + UI Stubs
- [x] Touch input capture stub
- [x] Pen/stylus input stub
- [x] Keyboard input stub
- [x] Action menu placeholder view
- [x] Add fullscreen MirrorActivity placeholder
- [x] Add Settings screen entry point from MainActivity
- [x] Add status chip styling for connection state
- [x] Add navigation to MirrorActivity from Connect button
- [x] Add Action Menu activity placeholder

## SonarPen Integration (Scaffold)
- [x] Add SDK module reference or dependency wiring
- [x] Add microphone permission flow stub
- [x] Add calibration activity placeholder
- [x] Add SonarPen status indicator placeholder
- [x] Add permission rationale copy for microphone access
- [x] Wire SonarPen SDK module when available

## Root / Magisk Module
- [x] Add Magisk module skeleton under `magisk/uberdisplay-root/`
- [x] Define root IPC contract (socket path, version, capabilities bitmap)
- [x] Add stub root daemon entrypoint and health check response
- [x] Document install/uninstall flow for the Magisk module
- [x] Add a `zip-module.ps1` helper script for packaging
- [x] Add basic daemon version/caps reporting in stub

## Testing
- [x] Add unit test source set and a basic test
- [x] Add instrumentation test skeleton
- [x] Add a test for root socket handshake parsing
- [x] Add a test for connection state transitions
- [x] Add a test for RootModuleStatus capabilities parsing
- [x] Add tests for ConnectionStateTracker edge cases

## Docs
- [x] Update `README.md` with Android build/run instructions
- [x] Document root module IPC in `spec.md` or a linked appendix
- [x] Add a brief Android folder overview to `readme.md`
