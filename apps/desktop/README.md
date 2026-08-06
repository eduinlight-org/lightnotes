# Desktop

The desktop crate is the entrypoint for the LightNotes desktop app. It renders the shared `app` crate through a webview and ships as a native application on macOS, Windows and Linux.

```
desktop/
├─ assets/              # Desktop specific assets (main.css, tailwind.css)
├─ icons/               # Application icon used by the bundler
├─ src/
│  └─ main.rs           # Entrypoint: mounts app::Route behind the desktop renderer
├─ entitlements.plist   # macOS hardened runtime entitlements used when signing
├─ Dioxus.toml          # Bundle metadata (identifier, publisher, icon)
└─ Cargo.toml
```

## Development

```bash
make app-desktop-dev
```

`API_BASE_URL` is read at **compile time** (`option_env!`), so it has to be set for the build, not for the run. `make` loads it from `.env`; copy `.env.dist` to `.env` to get the local default of `http://localhost:4000`.

## macOS

### Building locally

```bash
make app-macos-bundle
```

That calls `scripts/macos-bundle.sh`, which runs `dx bundle --package-types macos --package-types dmg` and renames the results to our release naming convention. Output lands in `dist/macos/`.

Signing and notarization are handled by `dx` itself, not by the script. `dx` reads the Apple credentials from the environment and does the whole sequence in the right order — it signs the frameworks, the binary and the `.app`, notarizes and staples the `.app`, then builds the DMG *from the already-signed app*, signs that, and notarizes and staples it too. The script only has to pass the environment through.

The script is driven entirely by environment variables, so CI and a local build take the same path:

| Variable | Default | Purpose |
| --- | --- | --- |
| `VERSION` | `dev` | Stamped into the artifact filenames |
| `ARCH` | host arch | `aarch64` or `x86_64`; also normalizes `arm64`/`amd64` |
| `TARGET` | host target | Rust target triple passed to `dx bundle` |
| `OUT_DIR` | `dist/macos` | Where the artifacts are written |

Cross-building for Intel from Apple Silicon:

```bash
rustup target add x86_64-apple-darwin
VERSION=0.1.0 ARCH=x86_64 TARGET=x86_64-apple-darwin make app-macos-bundle
```

We ship **one DMG per architecture** rather than a universal binary — `dx bundle` builds a single target at a time, and two separate downloads keep each one roughly half the size of a fat binary.

### Signing and notarization

Release builds are signed with a Developer ID Application certificate and notarized by Apple so users don't hit Gatekeeper. CI does this automatically when these repository secrets are present; without them the workflow still runs and produces an **unsigned** build.

These names are dictated by `dx` — it looks up exactly these variables, so don't rename them:

| Secret | What it holds |
| --- | --- |
| `APPLE_CERTIFICATE` | Developer ID Application certificate, exported as `.p12` and base64-encoded |
| `APPLE_CERTIFICATE_PASSWORD` | Password protecting that `.p12` |
| `APPLE_ID` | Apple ID used for notarization |
| `APPLE_PASSWORD` | App-specific password for that Apple ID (**not** the account password) |
| `APPLE_TEAM_ID` | Apple Developer team ID |

Setting `APPLE_CERTIFICATE` turns on signing: `dx` imports it into a temporary keychain and finds the identity. Setting `APPLE_ID` additionally turns on notarization. Signing without notarizing is a valid intermediate state, and the script warns when it detects it.

Export the certificate with:

```bash
security find-identity -v -p codesigning          # find the Developer ID Application identity
# export it from Keychain Access as certificate.p12, then:
base64 -i certificate.p12 | pbcopy                # paste into the APPLE_CERTIFICATE secret
```

Signing uses the hardened runtime with `entitlements.plist`, wired up through `[bundle.macos] entitlements` in `Dioxus.toml`. That path is passed straight to `codesign` and is resolved relative to the working directory, so it is written as `apps/desktop/entitlements.plist` — run `dx`/`make` from the repository root.

The JIT and unsigned-executable-memory entitlements are required: JavaScriptCore inside the webview will not start without them.

### CI

`.github/workflows/release-desktop-macos.yml` builds both architectures on `macos-latest` by calling the same script. It is triggered by `workflow_call` from the umbrella release workflow, and by `workflow_dispatch` for manual runs.

Artifacts are uploaded as `desktop-macos-<arch>` and named:

```
LightNotes-<version>-macos-aarch64.dmg
LightNotes-<version>-macos-x86_64.dmg
LightNotes-<version>-macos-<arch>.app.zip
```
