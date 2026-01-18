# Root Module IPC Contract (Draft)

This module exposes a local Unix socket for the Android app to query root
capabilities and request privileged operations.

## Socket
- Path: `/data/local/tmp/uberdisplay/root.sock`
- Transport: Unix stream socket
- Permissions: owned by `root:root`, mode `0660`
- Client: Android app connects locally (no network exposure)

## Protocol
Line-oriented ASCII messages terminated by `\n`.

Handshake:
1) Client sends: `HELLO 1\n`
2) Server responds: `OK 1 caps=0x00000000\n`

Health:
- Client sends: `PING\n`
- Server responds: `PONG\n`

Capabilities:
- Client sends: `CAPS\n`
- Server responds: `CAPS 0x00000000\n`

Disconnect:
- Client sends: `BYE\n`
- Server closes the connection.

## Capability Bits (draft)
- `0x00000001` input injection (low-latency)
- `0x00000002` usb/aoap tuning helpers
- `0x00000004` scheduler/perf tuning hooks
- `0x00000008` display pipeline hooks (device-specific)

## Notes
- This is a stub contract and will evolve with the actual root daemon.
- All operations must be explicitly gated in-app by a user setting.
