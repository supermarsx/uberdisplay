# UberDisplay — Product & Technical Specification

This document specifies **UberDisplay** based on the implementation currently present in `superdisplayded/` (an Android “mirror client” application with USB + Wi‑Fi transports, video decode/render, and bidirectional input).

If UberDisplay is intended to be wire‑compatible with the existing host/driver implementation, the protocol constants and packet formats described here should remain unchanged.

---

## 1) Summary

UberDisplay is a multi-component system that turns an Android device into an **external display + input surface** for a host computer.

Core capabilities:
- Receive a low-latency video stream from the host and render it fullscreen.
- Send touch, pen/stylus, and keyboard input back to the host.
- Connect via:
  - **USB (AOAP / Android Open Accessory)**, and/or
  - **Wi‑Fi (TCP server on device)**.
- Provide an **on-screen action menu** for quick commands and view/crop presets.
- Optional **foreground service** to remain connected in the background.
- Optional diagnostics/error reporting.

### System Components (Target)
- **Android app (Client)**: receives frames, renders, and sends input back.
- **PC app (Host agent)**: cross-platform desktop app that captures/encodes video and injects input. Implemented in **Tauri** with **Rust** backend and **Next.js** frontend.
- **Windows virtual display driver**: optional-but-recommended driver that creates a virtual monitor so apps can render to a simulated screen (similar in scope to `https://github.com/VirtualDrivers/Virtual-Display-Driver`).

Non-capabilities (as implied by the current client source):
- The Android app does not itself implement the host-side capture/encode “driver” or PC agent.
- The Android app does not discover hosts automatically (Wi‑Fi is via a listening TCP port; USB is via accessory attach/polling).

---

## 2) Terminology

- **Host**: the PC-side system (PC app/agent + any required drivers) sending video frames and receiving input events.
- **Client**: the Android app (UberDisplay).
- **Transport**: the underlying connection (USB AOAP pipe or TCP socket).
- **Session**: an active mirroring run from "start" to "stop" including decoding and input.
- **AOAP**: Android Open Accessory Protocol (USB accessory mode).
- **Crop / View**: a transform applied to the host surface used for "zoom/pan/rotate" style view control.
- **Virtual display driver**: OS driver that creates a simulated monitor (Windows target).

---

## 3) Goals and Non‑Goals

### Goals
- Low-latency, stable display mirroring with responsive input.
- USB-first workflow with optional Wi‑Fi workflow.
- Minimal on-device steps: connect, open UberDisplay, mirror starts, input works.
- Robust handling of disconnects, version mismatch, and “no video” scenarios.
- Configurable input modes, action menu, framerate/quality/resolution controls.

### Non‑Goals
- Implementing the host capture/encode pipeline inside the Android app.
- Remote internet streaming; the protocol is intended for local USB/Wi‑Fi.
- Strong network authentication (the reference implementation is effectively “LAN‑open”).

---

## 4) User Experience (High Level)

### Primary flows
1. **USB (recommended)**
   - User plugs Android device into host.
   - Host places device into AOAP accessory mode (host-side responsibility).
   - Client detects accessory, negotiates protocol versions, starts a session, displays video.

2. **Wi‑Fi (optional)**
   - User opens UberDisplay on device.
   - Host connects to device IP on port `1445` (default).
   - Session starts and video renders.

### Session UX
- Fullscreen “mirror” view (separate activity) with immersive UI.
- On-screen action menu (optional) for:
  - toggling input modes,
  - sending predefined keys/commands,
  - selecting and saving crop/view presets.
- Touch/stylus gestures forwarded to host; optional “view transform” gestures when in scale mode.
- Optional "remain connected" background behavior via a foreground service.

---

## 4.1) Design System (Target)

UberDisplay should present a cohesive, premium UI across **PC** and **Android** with:
- **Primarily dark mode** by default.
- **Stylish serif + script typography** for headers, with a clean sans for UI/body text.
- **Watercolor / paper texture styling** (soft washes, torn-paper edges, subtle grain), inspired by `https://untamedheroinegame.maxmara.com/`.

### Visual references (what to emulate)
From the reference vibe (e.g., the “Untamed Heroine” card):
- Muted watercolor washes (sage/olive/stone) with layered paper textures.
- Large high-contrast serif wordmark + a smaller italic/script accent line.
- Subtle “handwritten” background text as a texture layer (low contrast, non-distracting).

### Color palette (dark-first)
- **Base**: near-black with a warm tint (charcoal/ink rather than pure #000).
- **Surface**: deep slate/graphite with slight green/brown undertones.
- **Text**: warm off-white/ivory for primary; desaturated gray-green for secondary.
- **Accent**: muted sage/olive + a warm parchment highlight.
- **Status colors**: subdued (avoid neon); use saturation + luminance for contrast.

### Typography (open fonts recommended)
Use fonts that are available and redistributable (e.g., via Google Fonts):
- **UI/body**: `Inter` (readability, dense settings screens).
- **Display serif**: `Libre Baskerville` (brand headline / titles).
- **Script accent** (sparingly): `Parisienne` or `Allura` for “signature” headings and micro-brand moments.

Rules:
- Script font is for short accents only (1–3 words), never for dense UI labels.
- Prefer serif for hero titles; use sans for controls, data tables, and settings.

### Watercolor styling guidelines
- Use watercolor as **background layers** and section dividers (not as noisy UI chrome).
- Provide 2–4 watercolor “themes” (sage, sand, midnight-blue, charcoal-wash) and reuse consistently.
- Add subtle **paper grain** + **torn edge masks** on large panels/hero sections.
- Keep controls crisp and modern (Material-like) on top of textured surfaces.

### Components (PC + Android)
- **Status chips**: pill-shaped, low-saturation, icon + label + short state (“Connected”, “Driver OK”, “Encrypting”).
- **Primary CTA**: high-contrast button with soft shadow; hover/focus states use watercolor-tinted glow.
- **Panels/cards**: blurred/translucent surfaces over watercolor backgrounds; strong typography hierarchy.
- **Forms/toggles**: consistent labeling, helper text, and validation; avoid decorative fonts in forms.

### Accessibility
- Maintain WCAG-friendly contrast for body text and critical status indicators.
- Provide a “reduced texture” toggle for readability/performance.

---

## 5) Functional Requirements

### 5.1 Connectivity
- Support both:
  - **AOAP USB transport** triggered by `android.hardware.usb.action.USB_ACCESSORY_ATTACHED` and/or polling,
  - **TCP transport** accepting incoming connections on `1445`.
- Provide connection state machine for UI:
  - No cable / waiting / waiting AOAP / ready / ready over Wi‑Fi / update required / denied / error.
- Prefer a single active client session at a time (drop or reject additional connections).

### 5.2 Video Rendering
- Render host video frames into a full-screen view.
- Decoder operates on an H.264 Annex‑B style stream (SPS/PPS detection is present).
- Must handle “no video” (e.g., width/height 0 in config) and show a hint overlay.
- Support “unsupported decoder resolution” feedback (decoder flags).
- Provide screenshot capture of the rendered GL frame to external pictures directory (capability exists in the reference; wiring/triggering is implementation-dependent).

### 5.3 Input
Bidirectional input from device to host:
- Touch:
  - multi-pointer support, normalized coordinates.
- Pen/stylus:
  - standard stylus axes (pressure/tilt/rotation) when available,
  - a “pen emulator” mode for devices without full stylus support,
  - SonarPen support via the bundled SDK (requires microphone permission).
- Keyboard:
  - map certain Android keycodes to a compact protocol representation.
- Action menu buttons can send:
  - key actions (including modifier combinations),
  - “commands” (host-defined).

#### Android gesture capabilities (Target)
The Android app must provide configurable gesture controls with clear mode separation to avoid accidental input:

- **Touch gestures (direct input mode)**
  - Single-finger: direct pointer/touch injection (default).
  - Two-finger (configurable): secondary actions such as right-click, scroll, or pan (host-side mapping).
  - Long-press: context click (optional).

- **View transform gestures (scale/navigation mode)**
  - Pinch to zoom, two-finger drag to pan, two-finger rotate (optional).
  - These gestures must map to the `Scale`/view transform control plane and never inject “clicks” unless explicitly configured.
  - Provide a visible indicator when in view-transform mode.

- **System gestures safety**
  - Avoid fighting OS navigation gestures: support edge exclusion zones and configurable “safe margins”.
  - Provide an emergency “unlock/release input” gesture or on-screen control (e.g., three-finger tap or long-press on a corner).

- **Full configurability**
  - Gestures must be configurable in Settings (enable/disable, thresholds, mapping).
  - Allow per-session overrides from the PC app (negotiated and shown to the user).

#### SonarPen support (Required)
UberDisplay must fully integrate the SonarPen SDK that is present in the reference libraries to provide pen pressure and button support on devices that lack a high-quality native stylus stack.

Functional requirements:
- **Enable/disable**
  - SonarPen can be explicitly enabled in Android Settings (stylus type).
  - When enabled, request and verify `RECORD_AUDIO` permission and guide the user if denied.
- **Connection lifecycle**
  - Detect connected/disconnected SonarPen state changes and surface a clear status indicator during sessions.
  - Handle reconnects without requiring app restarts.
- **Pressure mapping**
  - Convert SonarPen raw amplitude into normalized pressure `[0..1]` using calibration values.
  - Provide smoothing/filtering options (off/low/medium/high) tuned for low-latency drawing.
- **Calibration**
  - Provide an in-app calibration screen (min/max amplitude capture) and store values persistently.
  - Support both:
    - manual calibration (sliders + live preview), and
    - guided calibration steps (“press lightly”, “press firmly”) with visual feedback.
  - Export/import calibration as part of config portability (with explicit user consent).
- **Button support**
  - Support SonarPen button events (sound-wave-detected button and/or key-event path) and map them into the existing `Pen` flags and/or configured actions.
- **Session integration**
  - When SonarPen is enabled, pen events should prefer SonarPen pressure over emulated pressure derived from touch size.
  - Clearly label the active stylus source in-session (Native / SonarPen / Emulated).

Non-functional requirements:
- **Latency**
  - Keep audio processing overhead minimal; prioritize responsiveness over heavy filtering.
- **Privacy**
  - Audio data must be used only for pen detection/pressure and never recorded or transmitted as audio.
- **Licensing**
  - Ensure redistribution and usage complies with the SonarPen SDK license terms.

### 5.4 Action Menu & View/Crop
- Configurable on-screen button menu (up to 10 buttons).
- Each button can have tap and hold actions (key/mouse/view/command).
- View presets:
  - Set active view (select preset).
  - Reset active view (clear preset / default transform).
- Gesture “scale mode” to control transform via two touch points.

### 5.5 Background Behavior (Remain Connected)
- If enabled, allow maintaining session while app is backgrounded using a foreground service (MediaProjection permission flow referenced by implementation).
- If disabled, closing/backgrounding the app should close the session.

### 5.6 Diagnostics & Error Reporting
- Optional user-controlled diagnostics (e.g., Sentry).
- Capture breadcrumbs for trace logging when enabled.
- Avoid uploading diagnostics unless opted in.

### 5.7 No Billing / Licensing
- UberDisplay does not implement billing, receipts, or licensing flows.

---

## 6) Android App Architecture (Reference Implementation)

## 6.0) Android App Architecture (Target) — Kotlin UI + Rust Core

UberDisplay’s Android app should be as close to native as possible, using **Kotlin** for UI/platform glue and **Rust** for performance-critical logic.

### Play Store compliance (Required)
The default Android distribution must be fully compatible with Google Play requirements:
- No root required and no assumptions about privileged permissions.
- Feature gating: any sensitive capability must be behind explicit user actions, visible settings, and OS permission prompts.
- No dynamic code loading or behavior intended to bypass Android security model.
- Clear disclosures for permissions (microphone for SonarPen, network, foreground service).
- All “advanced” capabilities that require root must be **absent** from the Play build (see Magisk module below).

### Optional Root/Magisk Module (Advanced, Separate Distribution)
All “lower level access” features must be provided via an **optional Magisk module** that is:
- Installed by the user independently (out-of-band, not shipped or fetched by the Play build).
- Strictly optional: the Android app must run fully without it, with graceful degradation.
- Clearly labeled as advanced/unsupported-by-Play and disabled by default.

#### Root module goals (what “lower level access” means)
When present, the module can unlock capabilities that are not reliably achievable in a Play-compliant app:
- **Lower-latency input injection**
  - Provide a privileged input injection path that reduces latency vs accessibility-based or app-local injection.
- **Deeper USB/AOAP integration**
  - More reliable USB device/accessory handling, recovery, and potentially higher throughput I/O paths where feasible.
- **Network and scheduling tuning**
  - Optional tuning for CPU affinity/priorities, socket buffers, and power/performance modes to stabilize high FPS.
- **Display / presentation hooks (device-dependent)**
  - Optional improvements for presentation timing or display pipeline integration where available on rooted devices.

Non-goals:
- The module must not attempt to exploit third-party apps or bypass DRM protections.
- The module must not silently collect data; all telemetry remains opt-in and local by default.

#### App ↔ module interface
The Android app must detect and integrate the module via a narrow, auditable interface:
- A single local IPC mechanism (e.g., a Unix domain socket / local TCP on `127.0.0.1` / binder-style service).
- Explicit handshake with:
  - module version,
  - capabilities bitmap,
  - permissions/feature flags enabled by the user.
- The app must refuse to use the module if:
  - version is incompatible,
  - signature/fingerprint check fails (optional hardening),
  - user has not explicitly enabled “Use root module” in settings.

#### Security & UX requirements
- Provide a “Root status” panel showing:
  - module installed/not installed,
  - module version and capabilities enabled,
  - last handshake time and errors.
- Provide a safe fallback path:
  - if module disappears mid-session, continue in non-root mode or end session gracefully with a clear message.
- Config portability:
  - root-only settings must export/import cleanly but remain inactive on non-root devices.

#### Release engineering separation
- Maintain separate Android build flavors:
  - `play` (no root features, Play-compliant)
  - `oss`/`power` (may include detection UI, but still does not bundle the module)
- Distribute the Magisk module via non-Play channels (e.g., GitHub releases), with its own updater and changelog.

#### Root IPC (Draft)
Root module IPC is a local Unix socket exposed at:
- `/data/local/tmp/uberdisplay/root.sock`

Line-oriented ASCII protocol (terminated by `\n`):
- Client: `HELLO 1`
- Server: `OK 1 caps=0x00000000`
- Client: `PING`
- Server: `PONG`

Capabilities bitmask (draft):
- `0x00000001`: input injection (low-latency)
- `0x00000002`: USB/AOAP tuning helpers
- `0x00000004`: scheduler/perf tuning hooks
- `0x00000008`: display pipeline hooks

### Rust-owned components (Android)
The following must live in Rust (shared with the PC app wherever possible):
- `protocol`: packet encode/decode, framing, session IDs, capability negotiation.
- `transport`: socket/USB I/O loops, buffering, backpressure, packet pacing.
- `security`: key exchange, AEAD encryption/decryption, replay protection, key rotation.
- `net-adaptation`: congestion signals, bandwidth estimation, adaptive bitrate/FPS/resolution decisions.
- `media-pipeline-control`: jitter buffer, depacketization, frame reordering, timing, stats.
- `compression`: non-video compression (e.g., LZ4/Zstd) and optional FEC primitives.
- `config`: config schema, migration, import/export, validation.
- `telemetry`: counters and performance metrics used by the UI (without heavy allocations on hot paths).

### Kotlin-owned components (Android)
- UI (Compose or Views), settings screens, onboarding, and accessibility.
- Android permission flows and OS integrations (camera for QR pairing, microphone permission for SonarPen, etc.).
- Video decode/render integration using platform-native APIs:
  - Prefer **MediaCodec** (and SurfaceView/TextureView) for video decode.
  - Rust provides the byte stream + timing; Kotlin/NDK bridges feed MediaCodec and drive presentation.
- Input capture from Android framework (MotionEvent/KeyEvent) and forwarding into Rust for packing/encryption/transmission.
- SonarPen SDK integration (Java/Kotlin), with pressure values forwarded into Rust for normalization/filtering and packetization.

### FFI boundary
- Use a stable Rust<->Kotlin bridge (JNI + a thin C ABI layer or UniFFI-style generated bindings).
- Minimize crossings on hot paths:
  - batch frame payloads and stats updates,
  - keep per-packet processing in Rust.

### Core components
- `com.kelocube.mirrorclient.App`
  - owns singletons: `TransportListener`, `Diagnostics`.
  - tracks foreground/background lifecycle (`isActive`).
  - decides whether to run `FgService` based on session activity + preference.
- `TransportListener`
  - arbitrates between `AOAPTransportListener` and `TCPTransportListener`.
  - exposes LiveData for UI state and “session active”.
- `MirrorClient`
  - wraps a `Transport`, reads typed packets, and owns a `MirrorSession`.
- `MirrorSession`
  - orchestrates video (`YUVVideoStream`), pointer overlay (`Pointer`), input (`Input`), action menu (`ActionMenu`).
  - manages config changes and session stop/start control messages.
- `Transport` (+ `TCPTransport`, `AOAPTransport`)
  - handshake, framing, read loop, send buffer, timeouts.
- UI activities/fragments
  - `MainActivity`: connection/status UI and entry point.
  - `MirrorActivity`: immersive mirroring session activity.
  - `SettingsActivity` / `SettingsFragment`: preferences.
  - `EditActionMenuActivity`: action menu customization.
  - `CalibrateStylusActivity`: SonarPen calibration UX.
- `FgService` (+ receiver): foreground service management while connected.

### Notable Third‑Party Libraries (Observed via imports)
The reference source includes integrations with the following external libraries (this list is not an exhaustive “Gradle dependency graph”, but it covers the non-Android/AndroidX libraries directly referenced in code):

- Material Components: `com.google.android.material.*` (UI components such as sliders).
- Material Dialogs: `com.afollestad.materialdialogs.*` (dialogs throughout UI flows).
- AboutLibraries: `com.mikepenz.aboutlibraries.*` (OSS license/about screen).
- Sentry: `io.sentry.*` (error reporting + breadcrumbs when enabled).
- LZ4: `net.jpountz.lz4.*` (pointer/cursor shape decompression).
- SonarPen: `com.greenbulb.sonarpen.*` (stylus pressure via audio; calibration UX).
- IconDialog/Icon packs: `com.maltaisn.icondialog.*`, `com.maltaisn.iconpack.defaultpack.*` (action menu icon picking).
- DragListView: `com.woxthebox.draglistview.*` (drag/drop ordering for action menu buttons).
- Preference widgets: `de.mrapp.android.preference.*` (custom preference components in Settings UI).

### Permissions and features
From the manifest (reference app):
- Network: `INTERNET`, `ACCESS_WIFI_STATE`, `ACCESS_NETWORK_STATE`.
- Audio: `RECORD_AUDIO` (needed for SonarPen).
- Foreground service: `FOREGROUND_SERVICE_MEDIA_PROJECTION`.
- GL ES 2.0 feature required.

---

## 6.1) PC App Architecture (Target) — Tauri (Rust) + Next.js

### Purpose
The PC app implements the “host-side” responsibilities that are not present in the Android client code:
- create/select a render target (preferably a virtual monitor on Windows),
- capture frames, encode video, stream frames to Android,
- receive and inject input into the OS/apps,
- provide a UI for setup, pairing/security, and session controls.

Additionally, the PC app must be able to act as a **receiver/client** so that **another PC can be used as a display** (PC-to-PC mode). This means the desktop app supports both roles:
- **Host mode**: capture + transmit.
- **Receiver mode**: receive + decode + present (fullscreen window or render into a virtual display), and optionally forward local input back to the host.

### High-Performance Transport + Media Pipeline (Target)
UberDisplay targets very high throughput and low latency over both USB and Wi‑Fi. The system must support multiple codecs and adaptive algorithms, and expose them as configurable options on both Android and PC.

#### Encryption and authentication (required)
UberDisplay must support secure sessions over both Wi‑Fi and USB, with configurable crypto modes:

- **Symmetric (shared-key) mode**
  - Use a modern AEAD cipher (e.g., **AES-256-GCM** or **ChaCha20-Poly1305**).
  - Key material sources:
    - **PSK**: user-provided pre-shared key (manual entry, QR, or copied token).
    - **Derived shared key**: produced by a pairing/handshake flow and stored locally.
  - Nonces must be unique per key; include replay protection (sequence numbers).

- **Asymmetric (public-key) mode**
  - Use authenticated key exchange to derive a session key, then encrypt all traffic with AEAD.
  - Supported options:
    - **Ephemeral ECDH (X25519)** with optional identity authentication.
    - Optional **certificate-based** identity (advanced/enterprise).
  - Authentication options:
    - “Trust-on-first-use” (TOFU) with device fingerprint confirmation.
    - PIN/QR-based pairing that pins a public key.

- **No-encryption mode (debug only)**
  - Allowed only behind an explicit debug flag with clear warnings.

Encryption requirements:
- Apply to **all transports** (Wi‑Fi TCP/QUIC and USB).
- Cover **video + input + control** messages (no “plaintext control channel”).
- Be **hardware-accelerated where available** (AES-GCM) but support ChaCha20-Poly1305 for devices without AES acceleration.

Configuration requirements (both Android + PC):
- crypto mode: `off (debug)` / `psk` / `public-key`
- cipher selection: `aes-gcm` / `chacha20-poly1305` (auto + manual override)
- pairing method: `pin` / `qr` / `manual key` / `tofu`
- key rotation policy (time-based or bytes-based) for long sessions

#### Transport options (recommended)
- **USB (primary)**:
  - Use **bulk transfers** with large, pipelined reads/writes and minimal copying.
  - Prefer a transport design that supports backpressure and avoids head-of-line blocking in the app layer.
- **Wi‑Fi (secondary)**:
  - Provide two modes:
    - **Low-latency UDP/QUIC mode** (preferred where available): application-level pacing + congestion control, optional FEC.
    - **TCP mode** (compatibility fallback): simpler but can suffer head-of-line blocking under loss.

#### Video codecs (best-effort priority order)
The PC app should select the best available codec based on hardware support (PC encoder + Android decoder), then fall back:
- **HEVC (H.265)**: best quality/bitrate on many modern GPUs + Android devices.
- **AV1**: excellent compression when both sides have hardware support; otherwise avoid for latency/CPU reasons.
- **AVC (H.264)**: widest compatibility; good low-latency baseline.
- **VP9**: last-resort fallback where H.264 is constrained or unavailable.

Low-latency encoder settings (guidelines):
- Disable B-frames (or use “low-delay” GOP structure).
- Prefer CBR/VBR with tight VBV (small buffer) and short keyframe interval.
- Support intra refresh (where available) to avoid full IDR spikes.

#### Compression (non-video payloads)
Compression is mandatory for large non-video assets and optional for other payloads:
- **Pointer/cursor shape**: LZ4 (already used in the reference client) or Zstd (optional) depending on CPU budget.
- **Config/metadata**: Zstd (optional) when beneficial.

#### Adaptive algorithms (required)
The system must implement adaptive strategies that can react in-session:
- **Adaptive bitrate**: target a latency budget by adjusting bitrate based on RTT, loss (Wi‑Fi), decoder queue depth, and frame pacing.
- **Adaptive resolution scaling**: step down/up resolution based on sustained bandwidth/decoder performance.
- **Adaptive framerate**: reduce FPS first under constraint to preserve interactivity, then reduce resolution/bitrate as needed.
- **Keyframe strategy**: request or schedule IDR/intra-refresh on major parameter changes or after packet loss recovery (Wi‑Fi).

#### Full configurability (both sides)
Both Android and PC apps must expose:
- codec selection (auto + manual override),
- bitrate (auto + manual cap),
- framerate target and caps,
- resolution mode (native/scale, explicit modes),
- transport mode (USB / Wi‑Fi TCP / Wi‑Fi QUIC/UDP),
- latency vs quality presets (e.g., “Gaming”, “Balanced”, “Quality”),
- advanced toggles (FEC on/off, keyframe interval, intra refresh, packet pacing).

Configuration must be:
- **negotiated** at session start (capability exchange),
- **changeable live** (mid-session reconfigure) where feasible,
- **observable** (UI shows effective values, not only requested values).

### Desktop App Requirements (Additional)
The desktop app must provide the following product-level controls and tooling:

- **Update mechanism**
  - In-app update checking and applying updates for the PC app (Tauri updater or equivalent).
  - Show current version, update channel (stable/beta), last check time, and update status.

- **Connectivity**
- **Connect via Wi‑Fi**: manual IP:port entry and saved profiles.
  - Provide connect/disconnect and reconnect controls with clear state feedback.
  - Pairing/security is recommended for Wi‑Fi (PIN/token) even if initial MVP is “trusted LAN”.
  - **Auto-find + auto-pair (optional)**
    - Discovery: PC app can automatically find nearby Android clients on the local network (mDNS/Bonjour and/or UDP broadcast discovery).
    - Auto-pair: PC app can complete pairing with minimal user steps:
      - “one-click pair” for devices advertising a short-lived pairing window/token,
      - or “pair with confirmation” requiring the user to confirm a matching code on Android and PC.
    - Discovery and auto-pair must be user-controllable (enable/disable) and should default to **safe** behavior (no silent trust of unknown devices).
    - For secured deployments, allow “auto-connect only to previously paired devices”.

- **AOAP / USB options (feature-gated)**
  - **Enable/disable AOAP** support in the PC app (global toggle).
  - AOAP may be Windows-only initially; UI must reflect availability per platform.

- **ADB options (feature-gated)**
  - **Enable/disable ADB** integration in the PC app (global toggle).
  - When enabled, optionally:
    - detect connected devices,
    - assist with USB debugging prerequisites,
    - use ADB as a helper for setup/diagnostics (not as a required runtime dependency unless explicitly chosen).

- **Android hotspot helper**
  - **Automatic Wi‑Fi IP** option when the Android device is acting as a hotspot / tethering endpoint.
  - The PC app should attempt to infer the Android device IP and/or provide a one-click “detect device IP” workflow (with fallbacks and clear instructions if OS blocks enumeration).

- **Wi‑Fi pairing via QR code (Android -> PC)**
  - Android app can display a pairing QR code that the PC app can scan (using the PC’s camera) to bootstrap a secure connection.
  - QR payload must support:
    - device IP (and optional hostname),
    - port(s) and preferred transport (`tcp`, `quic/udp`),
    - protocol version / required minimum version,
    - cryptographic bootstrap data:
      - PSK token *or* public-key fingerprint for pinning (depending on security mode),
      - optional short-lived pairing nonce/expiry to prevent replay.
  - PC app must provide:
    - “Scan QR” flow,
    - fallback manual entry,
    - clear confirmation UI showing the device identity/fingerprint before connecting (unless explicitly configured to auto-accept).

- **Windows pen stack option**
  - **WinTab driver option**: allow choosing between pen injection paths (e.g., WinTab vs Windows Ink / pointer injection), where supported.
  - Surface a capability check and explain tradeoffs (compatibility vs latency/precision).

- **Driver + general status controls**
  - Dedicated “Status” area with:
    - virtual display driver installed/healthy status,
    - virtual display instance present/active status,
    - capture pipeline status (capturing/idle/error),
    - encoder status (hardware/software, selected codec),
    - transport status (connected, RTT, throughput),
    - input injection status (enabled/blocked by OS permissions).

- **Labels, validation, and checks**
  - Clear labels for each feature toggle and dependency.
  - Preflight checks before “Start session” (driver present, permissions, selected monitor, reachable IP).
  - Human-readable error messages and suggested fixes (not just codes).

- **Debugging options**
  - Enable/disable verbose logging.
  - Export logs (and optionally a sanitized “support bundle” that includes config + recent logs).
  - Optional protocol tracing (packet headers/lengths; never record sensitive payloads by default).
  - Developer tools panel (feature flags, simulated disconnects, encoder fallback toggle) guarded behind an “advanced/debug” switch.

- **Config portability (PC + Android)**
  - Both apps must support **exportable/importable configurations**:
    - export to a single file (JSON by default; optionally YAML/TOML),
    - import with validation, migration, and safe defaults,
    - allow “partial import” (only networking, only video, only input mappings, etc.).
  - Configuration must include:
    - transport/security settings (without leaking secrets by default),
    - codec/bitrate/FPS/resolution presets,
    - device profiles and pairing records (exportable with explicit user consent),
    - UI/theme preferences,
    - action menu mappings and shortcuts.
  - Provide a “share config” flow:
    - Android: share sheet export + QR for small configs.
    - PC: save/export + copy-to-clipboard.

- **Autoprovisioning & scripting**
  - PC app must offer a **headless/CLI** interface for automation:
    - install/check virtual display driver (Windows),
    - create/remove virtual display,
    - apply a config file,
    - start/stop a session,
    - list paired devices and connect by name.
  - Support “autoprovision scripts”:
    - single command to set up driver + config + pairing in a repeatable way.
  - Android must support a limited automation surface where feasible:
    - deep links / intents to apply/import config and open pairing screen,
    - optionally a local HTTP endpoint in debug builds only.

- **Packaging and distribution (PC app)**
  - Provide official packages for:
    - **Windows**: `winget` + `scoop` (with a stable release channel).
    - **macOS**: `brew` cask.
  - Packaging must support:
    - silent install flags (where platform allows),
    - versioned upgrades and clean uninstalls,
    - post-install verification (driver status on Windows, app permissions guidance on macOS).

### Backend (Rust)
Recommended Rust modules/crates:
- `protocol`: implements handshake + packet encode/decode (mirror the spec’s DataTypes).
- `transport`: Wi‑Fi TCP connection management, discovery/pairing, reconnect logic.
- `capture`: platform capture abstraction (Windows first; others behind feature flags).
- `encode`: encoder abstraction (hardware-first, with software fallback).
- `input`: OS input injection (keyboard/mouse/stylus where possible).
- `virtual_display`: Windows-only integration layer for the driver component.
- `update`: update checking/applying, channel selection, and rollback-safe UX.
- `receive`: receiver pipeline (network receive, jitter buffering, decode, presentation, stats).

### Frontend (Next.js)
Screens:
- setup wizard (driver install status, permissions, pairing),
- connect screen (device IP, connect/disconnect, status),
- session config (resolution/FPS/quality/orientation),
- advanced (security, logs export).

---

## 6.2) Windows Virtual Display Driver (Target, Separate Component)

### Goal
Provide a virtual monitor so Windows and apps can render to a simulated screen even without a physical display connected.

### Baseline
Use an open-source virtual display driver similar to:
- `https://github.com/VirtualDrivers/Virtual-Display-Driver`

### Integration contract (PC app ↔ driver)
The PC app should be able to:
- install/uninstall (or guide installation),
- create/remove one or more virtual display instances,
- set/query supported display modes (resolution, refresh rate, orientation) per display,
- detect whether the virtual display is present and active.

Note: Driver build/signing/distribution is its own concern and must be handled separately from the Tauri app.

---

## 6.3) Monorepo Layout (Target)

This repository should be split so Android and PC are cleanly separated, with shared artifacts for protocol correctness:

- `android/`
  - Android client app (new UberDisplay client implementation)
- `pc/`
  - Tauri app (Rust backend + Next.js UI)
  - `pc/src-tauri/` (Rust)
  - `pc/app/` or `pc/ui/` (Next.js)
- `driver/`
  - `driver/windows/` (virtual display driver project; may be a submodule or fork)
- `shared/`
  - `shared/protocol/` (Rust crate for the wire protocol + test vectors)
  - `shared/core/` (Rust crates for transport, security, adaptation, stats; used by PC + Android)
  - `shared/spec/` (spec docs, diagrams, binary fixtures)

If the Android app is not Rust-based, share protocol correctness via fixtures (golden packets, capture logs) and keep the Rust `shared/protocol` crate as the source of truth for the PC app.

---

## 7) Transport & Wire Protocol Specification

### 7.1 Handshake
After opening the underlying byte stream:
- Client reads and validates:
  - `HANDSHAKE_BASE = "KELOCUBE_MIRR_"`
  - `HANDSHAKE_VERSION_LENGTH = 3`
  - Host sends: `KELOCUBE_MIRR_###\0` where `###` is ASCII decimal, zero-padded, and `\0` is a single null byte.
- Supported host versions (client-side reference): `{4, 3}` with preferred/latest `4`.
- Version rules:
  - If host version > client preferred => **client too old** (update client).
  - If host version < preferred and not in supported list => **host too old** (update host).
- After handshake, client sets **Little Endian** byte order for all buffers and payload parsing.

### 7.2 Framing: stream multiplexing (Host -> Client)
Host-to-client data is chunked into “streams” (buffers) to support large payloads:

Each chunk begins with:
- `stream_id` (`u8`)
- `chunk_len` (`u16le`)
- `chunk_bytes[chunk_len]`

The client appends chunk bytes into a per-stream accumulator buffer.

Within each stream accumulator, packets are delimited as:
- `packet_len` (`u32le`)
- `packet_bytes[packet_len]`

`packet_bytes` begins with a `data_type` byte (see below), followed by the type-specific payload.

Reference buffer sizes:
- Stream 0 buffer: `100_000` bytes.
- Stream 1 buffer: `10_000_000` bytes.

### 7.3 Framing: Client -> Host
Client-to-host packets are written into a single send buffer and emitted as:
- `packet_len` (`u32le`)
- `packet_bytes[packet_len]`

Reference maximum outgoing chunk:
- `packet_len + 4 <= 10_000` (so `packet_len <= 9996`).

### 7.4 Data Types
The client recognizes these `data_type` IDs:

| Name | ID | Typical direction | Purpose |
|---|---:|---|---|
| `State` | 0 | Host -> Client | Host state/notifications (details host-defined). |
| `Configure` | 1 | Host -> Client | Configure/renegotiate session (resolution, encoder id). |
| `Stop` | 2 | Client -> Host | Stop session. |
| `Frame` | 3 | Host -> Client | Video frame payload. |
| `FrameDone` | 4 | Client -> Host | Frame processed/ready ack. |
| `PointerMove` | 5 | Host -> Client | Pointer (cursor) position updates. |
| `PointerShape` | 6 | Host -> Client | Pointer shape image update. |
| `TakeScreenshot` | 7 | Host -> Client | Request client screenshot (implementation-dependent). |
| `Touch` | 8 | Client -> Host | Touch pointer events. |
| `Pen` | 9 | Client -> Host | Stylus/pen events. |
| `Unlock` | 10 | Reserved | Not used by UberDisplay (no billing/licensing). |
| `Scale` | 11 | Both | View transform / crop / scale gestures & updates. |
| `InputConfig` | 12 | Client -> Host | Input configuration (e.g., button function). |
| `InputKey` | 13 | Client -> Host | Action menu key events / mapped actions. |
| `Error` | 14 | Host -> Client | Host-reported error/warning. |
| `Keyboard` | 15 | Client -> Host | Keyboard key down/up events. |
| `Command` | 16 | Client -> Host | Host-defined "command" invocations. |
| `Capabilities` | 17 | Both | Codec + feature capability negotiation (vNext). |

### 7.5 Payload formats (as implemented by the client)

Unless otherwise stated, all integers are **Little Endian**.

#### `Configure` (Host -> Client)
Payload is parsed into:
- `width` (`i32`)
- `height` (`i32`)
- `hWidth` (`i32`) — host surface width (for pointer transforms)
- `hHeight` (`i32`) — host surface height
- `encoderId` (`i32`) — identifier echoed back in `FrameDone`
- **Optional v2 extension** (if payload length allows):
  - `codecId` (`u8`)
  - `codecProfile` (`u8`)
  - `codecLevel` (`u8`)
  - `codecFlags` (`u8`) — reserved (0 for now)

Codec IDs (host-selected):
- `1` = H.264
- `2` = H.265 / HEVC
- `3` = AV1
- `4` = VP9
- `5` = H.266 (reserved; negotiate only if both sides advertise support)

Notes:
- If `width == 0`, the client may tear down video output and show a “no video” hint.
- The on-device decoder is configured with `(width, height)`.

#### `Frame` (Host -> Client)
Client behavior:
- Skips **one additional byte** after `data_type` (an extra per-frame header byte).
- The remainder is passed to the decoder as codec stream bytes indicated by `codecId` from `Configure` (H.264 fallback if absent).

Payload layout (inferred from reads):
- `frame_meta` (`u8`) — currently unused on client, host-defined.
- `h264_bytes[...]`

#### `FrameDone` (Client -> Host)
- `encoderId` (`i32`)

#### `PointerMove` (Host -> Client)
- `x` (`i16`)
- `y` (`i16`)

#### `PointerShape` (Host -> Client)
- `width` (`i16`)
- `height` (`i16`)
- `hotSpotX` (`i16`)
- `hotSpotY` (`i16`)
- `lz4_rgba_bytes[...]` — LZ4-compressed RGBA image data intended to decompress to `width * height * 4` bytes.

#### `Touch` (Client -> Host)
Layout:
- `count` (`u8`)
- `points[count]` where each point is 8 bytes:
  - `pointerId` (`u8`)
  - `down` (`u8`) — `1` pressed/contact, `0` released for the up pointer
  - `x` (`i16`) — normalized [0..1] scaled by 32767
  - `y` (`i16`) — normalized [0..1] scaled by 32767
  - `size` (`i16`) — normalized size scaled by 32767

Notes:
- For ACTION_DOWN/ACTION_UP, `count` is `1` and only the action index is serialized.
- For move/pointer changes, `count` is `pointerCount` and all pointers are serialized.

#### `Pen` (Client -> Host)
Fixed 11 bytes:
- `flags` (`u8`)
  - bit 0: contact
  - bit 1: hover
  - bit 2: buttonDown
- `x` (`i16`) normalized [0..1] * 32767
- `y` (`i16`) normalized [0..1] * 32767
- `pressure` (`i16`) normalized [0..1] * 32767
- `rotation` (`i16`) normalized (stylus axis 8) * 32767, or `0` when emulated
- `tilt` (`i16`) normalized (stylus axis 25) * 32767, or `0` when emulated

Notes:
- In “emulated” mode, pressure is derived from touch size or SonarPen amplitude calibration.

#### `Scale` (Both)
The `Scale` data type is **multi-form**, keyed by a leading signed byte (`i8`):

Client -> Host forms (9 bytes total):
1) **Gesture state** (sent during “scale mode”)
   - `mode` (`i8`): `0..2` = number of pointers, `-2` = cancel
   - `p0x` (`i16`) packed coord
   - `p0y` (`i16`) packed coord
   - `p1x` (`i16`) packed coord
   - `p1y` (`i16`) packed coord

2) **Set crop** (sent when selecting/saving a crop)
   - `mode` (`i8`): `-1`
   - `x` (`i16`) packed crop x
   - `y` (`i16`) packed crop y
   - `a` (`i16`) packed crop scale factor
   - `t` (`i16`) packed crop rotation factor

Host -> Client form (9 bytes total):
- `x` (`i16`)
- `y` (`i16`)
- `a` (`i16`)
- `t` (`i16`)
- `target` (`u8`) — nonzero indicates “save” target vs “preview” (drives different UI behavior).

“Packed coord” uses a short mapping of a float in [0..1] to an `i16` range:
- `packed = -32768 + round(x * 65535)` (reference behavior)

#### `InputConfig` (Client -> Host)
- `buttonFunction` (`i32`) — a packed action id used by the host to interpret a device “button”.

#### `InputKey` (Client -> Host)
Sent by the action menu to represent a key/action press originating from a specific menu button:
- `down` (`u8`) — 1 down, 0 up
- `buttonIndex` (`u8`) — index of action menu button
- `action` (`i32`) — packed action id (includes key/mouse/command type and modifiers)

#### `Keyboard` (Client -> Host)
- `down` (`u8`) — 1 down, 0 up
- `keyIndex` (`i32`) — index into a shared key list (`KeyCodesKt.KEY_CODES` mapping in reference)

#### `Command` (Client -> Host)
- `commandId` (`i32`) — host-defined command index.

#### `Capabilities` (Both)
Payload (Little Endian):
- `codecMask` (`u32`) — bitmask of supported codecs.
  - bit 0: H.264
  - bit 1: H.265 / HEVC
  - bit 2: AV1
  - bit 3: VP9
  - bit 4: H.266 (reserved; advertise only if supported)
- `flags` (`u32`) — feature flags (vNext, currently 0).
- Optional fields may be appended in future versions.

Negotiation rule (Windows-first):
- Host selects codec in priority order: **H.265 HEVC → AV1 → H.264 → VP9** based on the intersection of host + client `codecMask`.
- Host sends selected `codecId` in `Configure` v2 extension.

#### `Unlock` (Reserved)
UberDisplay does not implement the `Unlock` flow.

#### `Error` (Host -> Client)
Payload begins with an error code byte which is mapped to:
- fatal errors (driver/license/trial/encoder/GPU),
- warnings (bad resolution / software encoder).

---

## 8) Capability Negotiation and Adaptive Control (Target)

The reference Android client’s `sendStart` payload is not fully recovered, so UberDisplay should define a clean vNext negotiation that supports high-performance streaming while remaining wire-compatible where needed.

### Requirements
- Capability exchange at session start:
  - supported transports (USB, TCP, QUIC/UDP),
  - supported codecs and profiles/levels,
  - max decode resolution/FPS on Android,
  - supported control features (dynamic reconfigure, IDR request, FEC).
- Security capabilities must be negotiated:
  - supported crypto modes (psk/public-key),
  - supported AEAD ciphers,
  - pairing method availability (qr/pin/tofu),
  - whether the transport already provides encryption (e.g., QUIC/TLS) and whether application-layer encryption is still enabled.
- Session parameters must be negotiable and adjustable:
  - codec, bitrate, FPS, resolution, keyframe interval, intra-refresh, latency mode.
- Adaptive control loop:
  - PC app is the control-plane “brain” by default (it observes network + encoder + capture).
  - Android app reports decode performance signals (frame time, queue depth, drops) to drive adaptation.

---

## 8.1) Multi-Display / Multi-Session Support (Target)

UberDisplay must support multiple virtual displays and multiple Android clients (where hardware allows), with explicit user control.

### Goals
- Create and manage **N virtual displays** (Windows) via one or more installed driver instances.
- Connect **multiple Android devices**, each assigned to:
  - a dedicated virtual display, or
  - a selected physical display, or
  - a “mirror/clone” mode (optional).
 - Connect **multiple PC receivers**, each assigned similarly (PC-to-PC “display endpoints”).

### Requirements (PC app)
- **Display inventory**
  - List physical + virtual displays with clear labeling (name, resolution, refresh, orientation, source driver).
  - Show driver provenance for each virtual display (“Driver A”, “Driver B”) when multiple driver backends are supported.
- **Virtual display manager**
  - Create/remove displays, rename labels, set default modes.
  - Enforce safe limits and warn about GPU/encoder constraints.
- **Per-display sessions**
  - Allow one active streaming session per display by default.
  - Optionally allow multi-stream per display (advanced; depends on encoder/capture pipeline).
- **Per-session configuration**
  - Each session can have its own codec/bitrate/FPS/resolution/transport/security settings.
  - Global defaults apply to new sessions, but sessions override locally.
- **Status and diagnostics**
  - Per-session status (connected, throughput, RTT/loss, dropped frames, encoder load).
  - Per-display capture status and driver health.

### Requirements (Android app)
- Support being paired to multiple PC “hosts” (profiles).
- If a single Android device can run multiple sessions simultaneously, it must:
  - provide a session switcher,
  - isolate input routing per session.
  - (MVP: allow only one active session per Android device.)

### Protocol implications
- Introduce a **session id / display id** concept in the vNext control plane so the PC can target configuration and stream selection per display/session.

---

## 8.2) PC-to-PC “Use Another PC as a Display” (Target)

UberDisplay must support using a second PC as a display endpoint by integrating a full receiver path into the PC app.

### Receiver modes
- **Fullscreen receiver**: present the stream in a low-latency fullscreen window on the receiving PC.
- **Virtual-display receiver (Windows)**: optionally render the received stream into a virtual display instance so other apps can “see” it as a monitor.

### High refresh rate support
- Receiver endpoints (PC and Android) must support high refresh rate presentation when hardware allows (e.g., **90/120/144/165/240 Hz**).
- The effective refresh rate must be **capability-driven**:
  - detect the target display’s supported refresh rates and current mode,
  - negotiate a target FPS/refresh with the host,
  - clamp to encoder/capture limits and transport conditions.
- Provide user controls:
  - `Auto` (use highest stable refresh),
  - explicit caps (60/90/120/144/165/240 where available),
  - “latency mode” preset that prioritizes input-to-photon time over absolute visual quality.
- Adaptive behavior:
  - If bandwidth/latency worsens, drop refresh rate smoothly (e.g., 120→90→60) before dropping resolution where appropriate.
  - Prefer consistent frame pacing; avoid unstable oscillation by using hysteresis and minimum hold times.

### Input forwarding (optional but recommended)
- Receiver PC can capture local input (mouse/keyboard/pen) and forward it back to the host PC.
- Must include clear UX for:
  - “capture input” vs “view only”,
  - an emergency escape (hotkey) to release input capture.

### Discovery & pairing
- Receiver PC participates in the same discovery/pairing mechanisms as Android:
  - auto-find, QR/pin pairing, and previously paired device lists.

### Security and performance
- Receiver PC must support the same encryption and adaptive streaming features as Android.
- Include jitter buffer controls (especially for Wi‑Fi/UDP) and performance telemetry.

### Backward compatibility
- If connecting to an older host/client, fall back to the legacy parameter set and disable unsupported adaptive features.

---

## 8) Settings / Preferences (Reference Keys)

Settings surfaced in `superdisplayded/app/src/main/res/xml/root_preferences.xml` and `PrefsKt`:

### Video
- `orientation` (list): default `landscape`.

### Input
- `stylus_type` (list): default `0`; value `2` enables SonarPen.
- `stylus_calibrate` (action): opens calibration UI (shown when relevant).
- `button_func` (dialog): packed action id; default `201` (none/neutral mapping).

### Action menu
- `session_menu_show` (switch): default `true`.
- `session_menu_modes` (multi-select): default from array resource.
- `session_menu_customize` (action): opens action menu editor.
- internal: `session_menu_active_crop` (int), `session_menu_onboarded` (bool).

### Connection
- `autoconnect` (switch): default `true`.

### Diagnostics
- `allow_error_reports` (switch): default is defined by the preferences UI (reference `root_preferences.xml` sets `true`, while code paths may still treat a missing key as `false`).

### Advanced
- `sampling` (list): default `0`.
- `quality` (seekbar 0–100): default `80`.
- `resolution` (list): host-driven entries expected.
- `framerate` (list): default `60`.
- `pref_force_software_encoder` (switch): default `false`.
- `test_id` (number): default `0` (controls decoder selection in reference).
- `ip_address` (info): displays device Wi‑Fi IP.
- `remain_connected` (switch, referenced): default `true`.

---

## 10) Diagnostics, Privacy, and Data Handling

### Diagnostics
- Diagnostics are toggleable via preference.
- When enabled, the reference uses Sentry DSN and records breadcrumbs for trace logs.

### Screenshots
- Client can save GL frames to `Pictures/mirror-client/<timestamp>.png`.

### Network exposure
- Wi‑Fi transport listens on a fixed port (`1445`).
- No authentication is implied by the protocol; LAN attackers could potentially connect and control behavior.

Recommended hardening for UberDisplay:
- Add pairing/authentication (PIN, QR, or shared secret) for Wi‑Fi sessions.
- Consider restricting Wi‑Fi server to foreground-only or same-subnet checks.

---

## 11) Performance & Reliability Requirements

- Target framerate: up to 60 FPS (configurable).
- Latency: prioritize low-latency decode/render; disable vsync where possible for presentation timing.
- Robustness:
  - time out on broken sockets,
  - surface recreation should not crash decode pipeline,
  - prevent send buffer overflow and enforce maximum packet size.

---

## 12) Compatibility Constraints (Reference)

- Android:
  - `minSdkVersion 21`
  - `targetSdkVersion 34`
  - `compileSdkVersion 34`
- Requires OpenGL ES 2.0.
- SonarPen requires microphone permission (`RECORD_AUDIO`).

---

## 13) Open Questions / To‑Dos for UberDisplay

These items cannot be fully specified from the client code alone:
- **Host-side `State` and `TakeScreenshot` payload formats**: client has placeholders but does not define them.
- **Exact `sendStart` payload**: `MirrorSession.sendStart()` is not decompiled in the reference; host/client start negotiation fields (quality/framerate/resolution/sampling/orientation/etc.) need confirmation from host source.
- **Security model**: decide on authentication/encryption for Wi-Fi.
- **Resolution list population**: reference UI expects a set of supported resolutions, likely delivered from host.

Additional items introduced by the target architecture:
- **PC capture pipeline**: choose capture tech per OS (Windows-first) and define supported encoders (hardware/software fallback).
- **Driver integration**: decide the exact API/IPC surface between the PC app and the Windows virtual display driver.
- **Cross-platform parity**: define what “virtual display” means on macOS/Linux (or explicitly scope it to Windows initially).

---

## 14) Traceability (Source References)

Key reference files that informed this spec:
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/Transport.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/TransportKt.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/MirrorClient.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/MirrorSession.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/Input.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/CanvasScaler.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/Pointer.java`
- `superdisplayded/app/src/main/java/com/kelocube/mirrorclient/ActionMenu.java`
- `superdisplayded/app/src/main/res/xml/root_preferences.xml`
- `superdisplayded/app/src/main/res/xml/accessory_filter.xml`
