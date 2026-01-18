# Repository Guidelines

## Project Structure & Module Organization
This repo is a reference + spec workspace for UberDisplay.
- `android/` holds the Android app project (Gradle, app source in `android/app/src/main/`).
- `libs/sonarpen-sdk/` contains the SonarPen SDK and sample Android projects.
- `spec.md` is the authoritative product and protocol specification.
- `_inspo_social-card.jpg` is a visual reference asset.

## Build, Test, and Development Commands
Run commands from the repo root unless noted.
- `cd android; gradle assembleDebug` builds a debug APK (or `.\gradlew` once a wrapper is added).
- `cd android; gradle installDebug` installs the debug APK on a connected device.
- `cd android; gradle clean` removes build outputs.
If you add tests, wire them into Gradle (for example, `gradle test` or `gradle connectedAndroidTest`).

## Coding Style & Naming Conventions
- Language: Kotlin with Java 8 target compatibility configured in `android/app/build.gradle`.
- Indentation: follow existing file style; default to 4 spaces for Kotlin/Gradle.
- Android package namespace is `com.supermarsx.uberdisplay`; keep new classes under this package unless intentionally modularizing.
- Avoid introducing new formatting tools unless they are applied consistently across the app.

## Testing Guidelines
No tests are currently defined in this repo.
- If you add unit tests, place them under `android/app/src/test/java/`.
- If you add instrumentation tests, place them under `android/app/src/androidTest/java/`.
- Name tests with a clear suffix, e.g., `TransportTest` or `MirrorSessionTest`.

## Commit & Pull Request Guidelines
Recent history uses short, imperative subject lines (for example, “Add …”).
- Keep commit subjects concise, start with a verb, and avoid punctuation.
- PRs should describe the change, point to the spec section in `spec.md` when behavior changes, and include screenshots for UI changes.

## Security & Configuration Tips
- USB and Wi-Fi transports are described in `spec.md`; if you change protocol behavior, update the spec alongside code.
- The SonarPen SDK in `libs/sonarpen-sdk/` may carry licensing constraints; verify before redistributing.
