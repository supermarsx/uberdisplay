# UberDisplay Root Module (Magisk)

This is an optional Magisk module that provides advanced capabilities for
UberDisplay on rooted devices. It is disabled by default and not required for
the Android app to run.

## Install
1) Zip the `magisk/uberdisplay-root/` folder contents into a Magisk module zip.
2) Install the zip from the Magisk app.
3) Reboot if Magisk prompts you to do so.

## Uninstall
- Remove the module from the Magisk app and reboot if prompted.

## Notes
- The root daemon is currently a stub. The IPC contract lives in `IPC.md`.
- Do not ship this module with Play Store builds.
