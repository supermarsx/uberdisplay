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
- [x] Add action menu edit screen stub
- [x] Add action menu persistence (prefs/json)
- [x] Add action menu edit list stub
- [x] Add basic transport connect/disconnect state in MirrorActivity
- [x] Add session status display in MirrorActivity
- [x] Add connection button state handling for WAITING
- [x] Add status chip color legend in settings
- [x] Add connection mode selector (TCP/AOAP) in settings
- [x] Add simple connection mode indicator in main screen
- [x] Add preference change listener to refresh mode label
- [x] Add basic transport status in settings (TCP port, AOAP)
- [x] Add TCP listener stub to update AOAP/TCP status
- [x] Add TCP status summary from TransportStatus
- [x] Refresh transport status when settings screen opens
- [x] Add TCP listener thread to accept connections
- [x] Show TCP connection count in settings
- [x] Add INTERNET permission for TCP transport
- [x] Add framed packet writer for outgoing packets
- [x] Write Keyboard/Command packets in SimplePacketWriter
- [x] Send keyboard packets from InputSenderStub
- [x] Add tests for framed packet writer
- [x] Add stream chunk writer for outgoing TCP
- [x] Wrap outgoing packets in stream chunks
- [x] Add tests for stream chunk writer
- [x] Split oversized payloads into multiple chunks
- [x] Parse State/Error/FrameDone packets in SimplePacketReader
- [x] Add tests for new packet parsing

## Next Focus
- [x] Add TCP packet reader loop skeleton
- [x] Add protocol framing parser for stream chunks
- [x] Add basic AOAP listener status signaling
- [x] Implement StreamBuffer.readPacket framing
- [x] Parse stream_id + chunk_len framing in TcpPacketLoop
- [x] Add basic heartbeat/keepalive placeholder
- [x] Add TCP connection timeout handling
- [x] Add StreamChunkParser tests
- [x] Add TCP outgoing packet queue stub
- [x] Add AOAP attach broadcast receiver stub
- [x] Wire TcpSenderQueue into InputSenderStub
- [x] Add AOAP status summary update on attach
- [x] Add AOAP detach receiver stub
- [x] Add last TCP connection time in settings summary
- [x] Add shared TCP outbox queue
- [x] Add TCP sender loop stub
- [x] Add sender queue drain tests
- [x] Add zero-length StreamBuffer test
- [x] Add ConnectionController stop test

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
- [x] Add protocol packet data models
- [x] Add packet reader/writer stubs
- [x] Add protocol framing helper stubs
- [x] Add packet parsing skeleton for Configure/Frame
- [x] Add packet writer skeleton for Touch/Pen
- [x] Add protocol tests for SimplePacketReader/Writer
- [x] Add transport/packet wiring in InputSenderStub
- [x] Add connection controller tests
- [x] Add protocol writer tests for zero packets
- [x] Add basic transport status refresh on connection start/stop
- [x] Add TCP port constant in settings summary from ProtocolConstants
- [x] Add settings hint for connection mode usage
- [x] Add mirror screen info banner placeholder
- [x] Add session start/stop buttons in MirrorActivity
- [x] Add session state persistence on rotate
- [x] Add FrameDone writer in SimplePacketWriter
- [x] Add tests for FrameDone writer
- [x] Add ActionMenuRepository serialization test
- [x] Add add-item action in ActionMenuEditActivity
- [x] Add handshake parser/helper
- [x] Add handshake parser tests
- [x] Add StreamChunkParser test for multiple packets
- [x] Add StreamBuffer multiple packet test
- [x] Add handshake parsing in TcpPacketLoop
- [x] Add InputKey/InputConfig packet models and writers
- [x] Add tests for InputKey/InputConfig writer
- [x] Send action menu item as InputKey packet
- [x] Send InputKey down+up for action menu tap
- [x] Send InputConfig for action menu button function
- [x] Add action menu sender tests
- [x] Add action menu item edit (rename/action id)
- [x] Add action menu edit tests

## PC App (Host)
- [x] Create `pc/` scaffold with Tauri backend + Next.js app router
- [x] Add initial UI shell styled to the spec (dark watercolor theme)
- [x] Add Rust protocol helpers for handshake and stream chunk framing
- [x] Add unit tests for host handshake and stream chunk framing
- [x] Wire Tauri `app_status` command into the UI status cards
- [x] Add transport status model (USB AOAP, TCP listen/connect) in Rust
- [x] Implement host->client packet builder for Configure/Frame/State
- [x] Add client->host packet parser for Touch/Pen/Keyboard/InputKey
- [x] Add host settings model (codec, bitrate, resolution, refresh cap)
- [x] Add device registry storage + persistence for paired Android devices
- [x] Add device registry CRUD commands (add/update/remove) exposed via Tauri
- [x] Wire device registry CRUD into the PC UI (pair/remove)
- [x] Add device edit UI (rename/status/transport)
- [x] Add Windows driver probe stub + UI status surface
- [x] Implement Windows driver detection (virtual display enumeration) + active display query
- [x] Add action menu quick buttons in MirrorActivity
- [x] Limit action menu buttons to 10 (spec)
- [x] Refresh ActionMenuActivity count on resume
- [x] Add long-press action menu button to send config
- [x] Add long-press config toast in MirrorActivity
- [x] Add TCP status label to MirrorActivity
- [x] Show TCP outbox size in settings
- [x] Show TCP packet counters in settings
- [x] Add transport counter reset action in settings
- [x] Add transport status reset test (unit)
- [x] Enforce max 10 action menu items in repository
- [x] Add test for ActionMenuRepository max items
- [x] Add transport summary line in MainActivity
- [x] Expand Touch packet writer to include points per spec
- [x] Expand Pen packet writer to spec fields
- [x] Build Pen packet from MotionEvent axes
- [x] Add Pen writer tests
- [x] Add multi-point Touch writer test
- [x] Send FrameDone after receiving Frame when encoderId known
- [x] Add StreamChunkParser test for FrameDone enqueue
- [x] Add periodic transport summary refresh in MainActivity
- [x] Add periodic TCP status refresh in MirrorActivity
- [x] Add TcpPacketLoop incomplete chunk test (parser level)
- [x] Add Pen writer flag byte test
- [x] Add SonarPen status label in MainActivity
- [x] Clamp touch size normalization
- [x] Add action menu config send in settings
- [x] Add action menu config send toast when empty
- [x] Show root caps in MainActivity status
- [x] Add RootModuleStatus handshake caching
- [x] Add RootModuleStatus cache test

## Codec + Streaming Pipeline
- [x] Define codec capability model + negotiation payload (HEVC/AV1/H.264/VP9) and extend Configure packet
- [x] Add protocol versioning/migration for codec negotiation in `spec.md`
- [ ] Implement host encoder abstraction with codec priority: H.265 HEVC -> AV1 -> H.264 -> VP9
- [ ] Wire GPU encoder backends where available (NVENC/AMF/QSV) with Media Foundation fallback on Windows
- [ ] Implement cross-platform capture + encode pipeline (Windows first, feature-gated for macOS/Linux)
- [ ] Add transport stream sender for encoded frames + FrameDone handling
- [ ] Implement Android decoder selection via MediaCodec with SurfaceView/TextureView fallback
- [ ] Add Android codec capability discovery + reporting to host
- [ ] Add end-to-end session start/stop pipeline (host capture/encode -> transport -> Android decode/render)

## PC Host Implementation (Windows-first)
- [x] Add host TCP reader loop for Android `Capabilities` + `FrameDone`
- [x] Persist negotiated codec + encoder backend in session state
- [ ] Implement Media Foundation encoder path for H.264/H.265 (baseline)
- [ ] Add GPU SDK probes for NVENC/AMF/QSV and map to encoder backend selection
- [ ] Wire capture source (DXGI Desktop Duplication) to encoder pipeline
- [ ] Emit `Configure` v2 and `Frame` packets over TCP session
- [ ] Add session health telemetry (fps, bitrate, queue depth) for UI diagnostics
- [ ] Add session stop + reconnect handling on socket failure

## Advanced Codec Support (EVC / MPEG-5)
- [ ] Add encoder backend evaluation for EVC (xeve) and MPEG-5 LCEVC
- [ ] Define codec profile/level mapping for EVC/LCEVC in Configure v2
- [ ] Add host-side software fallback stubs for EVC/LCEVC (no-op until real encoder wired)
- [ ] Update Android decoder capability probing for EVC/LCEVC if MediaCodec exposes support

## PC Pipeline Buildout (Detailed)
- [ ] Build host TCP session manager (connect, handshake, negotiate, configure, send loop)
- [ ] Add session lifecycle states + error transitions for UI (idle/connecting/streaming/error)
- [ ] Implement Media Foundation H.264 encoder with low-latency settings (no B-frames, short GOP)
- [ ] Implement Media Foundation H.265 encoder (if supported)
- [ ] Add encoder parameter controls (bitrate/fps/keyframe interval) mapped from settings
- [ ] Add DXGI capture surface and GPU-friendly frame upload path
- [ ] Implement frame pacing loop with backpressure using FrameDone ack
- [ ] Add stats sampling (encode time, send time, queue depth, RTT proxy)
- [ ] Add session logs + export hooks for pipeline stats

## Media Foundation Baseline (Work Started)
- [ ] Implement Media Foundation encoder init + output for H.264
- [ ] Implement Media Foundation encoder init + output for H.265 (if available)
- [ ] Map encoder settings (bitrate, fps, keyframe interval) from HostSettings
- [ ] Replace dummy frame sender with encoded output
 - [x] Feed frame pacing from FrameDone ack + fallback to fps timer when no acks

## Remote Input Controls (PC UI + Host)
- [x] Add input capture controls (enable/touch/pen/keyboard) to PC session settings
- [x] Add host-side input filter flags and apply to incoming packets
- [x] Add per-device input permissions and persistence
- [ ] Add command packet to toggle input modes per session
