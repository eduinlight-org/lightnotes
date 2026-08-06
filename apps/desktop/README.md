# Desktop

The desktop crate is the entrypoint for the LightNotes desktop app. It renders the shared `app` crate through a webview and ships as a native application on macOS, Windows and Linux.

```
desktop/
├─ assets/              # Desktop specific assets (main.css, tailwind.css)
├─ icons/               # Application icons used by the bundler, 32px through 1024px
├─ src/
│  └─ main.rs           # Entrypoint: mounts app::Route behind the desktop renderer
├─ entitlements.plist   # macOS hardened runtime entitlements used when signing
├─ Dioxus.toml          # Bundle metadata (identifier, publisher, icons, per-platform settings)
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

The script **unsets** any of these that are empty before invoking `dx`. That is not tidiness — it is load-bearing. `dx` decides whether to sign with `std::env::var("APPLE_CERTIFICATE").ok()`, and a variable that is *defined but empty* returns `Some("")`, not `None`. A workflow that writes `APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}` for a secret that doesn't exist defines exactly such a variable, so `dx` would try to import an empty certificate and die with `SecKeychainItemImport: One or more parameters passed to a function were not valid`. Unsetting is what makes "no secrets configured" mean "unsigned build" instead of "failed build".

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

## Windows

### Building locally

```bash
make app-windows-bundle
```

That calls `scripts/windows-bundle.ps1`, which runs `dx bundle --package-types msi --package-types nsis` and renames the results to our release naming convention. Output lands in `dist/windows/`. It has to run on Windows — there is no cross-build path from macOS or Linux.

`dx` downloads WiX and NSIS itself on first run, so no toolchain setup is needed beyond Rust and the MSVC target.

The script is driven by environment variables, so CI and a local build take the same path:

| Variable | Default | Purpose |
| --- | --- | --- |
| `VERSION` | `dev` | Stamped into the artifact filenames |
| `ARCH` | `x86_64` | `x86_64` or `aarch64`; also normalizes `x64`/`amd64`/`arm64` |
| `TARGET` | `x86_64-pc-windows-msvc` | Rust target triple passed to `dx bundle` |
| `OUT_DIR` | `dist\windows` | Where the artifacts are written |
| `WINDOWS_CERTIFICATE` | unset | Base64-encoded `.pfx`; unset means unsigned installers |
| `WINDOWS_CERTIFICATE_PASSWORD` | unset | Password protecting that `.pfx` |
| `WINDOWS_TIMESTAMP_URL` | DigiCert | RFC 3161 timestamp server |

We ship `x86_64` only for now. `aarch64-pc-windows-msvc` is a one-line matrix addition when there's demand — Windows on ARM runs x64 binaries under emulation in the meantime.

### Signing

Signing runs through `[bundle.windows.sign_command]` in `Dioxus.toml`, which points `dx` at `scripts/windows-sign.ps1`. `dx` invokes that script once per artifact it wants signed, passing the path.

The script exits successfully without doing anything when `WINDOWS_CERTIFICATE` is unset, so the same configuration produces unsigned builds on a machine with no certificate and signed builds in CI once the secret is set. It resolves `signtool.exe` from `PATH` or from the Windows SDK, writes the decoded `.pfx` to a temp file, and signs with SHA-256 plus an RFC 3161 timestamp.

Repository secrets:

| Secret | What it holds |
| --- | --- |
| `WINDOWS_CERTIFICATE` | Code-signing certificate exported as `.pfx` and base64-encoded |
| `WINDOWS_CERTIFICATE_PASSWORD` | Password protecting that `.pfx` |

**Known gap:** `dx` only signs the final MSI and NSIS installers — it does not sign the application `.exe` that gets installed. It copies the binary into its staging directory and signs only the installer it produces afterwards, with no hook in between. The installers are what SmartScreen evaluates on download, so this covers the case that matters most, but the installed executable shows no publisher in UAC. Signing it too needs an upstream change in `dx` or a rebuild of the staging step.

### WebView2

The Windows webview needs the WebView2 runtime. We use `webview_install_mode = { OfflineInstaller = { silent = true } }` — the runtime installer is embedded in the NSIS installer and runs silently if the machine doesn't already have it.

This is `dx`'s default, and we set it explicitly so the choice is visible. It trades a larger download for an install that works on a machine with no internet access and never fails partway through fetching a bootstrapper. Windows 11 ships WebView2, so most users never trigger it. Switch to `{ DownloadBootstrapper = { silent = true } }` if installer size becomes the bigger concern.

### Install mode and upgrades

`install_mode = "CurrentUser"` — installs per-user into the user's profile, so no admin prompt.

`[bundle.windows.wix] upgrade_code` is pinned to `6e851b49-e3ee-5439-8aa4-8209e2386b03`. This GUID **must never change**: Windows uses it to recognize that a new MSI upgrades an existing install rather than being a different product. The pinned value is the same one `dx` derives by default today (a UUIDv5 of `LightNotes.exe.app.x64`), so pinning it changes nothing now — it just stops the code from silently shifting if the product name is ever edited, which would leave users with two parallel installs.

### CI

`.github/workflows/release-desktop-windows.yml` builds on `windows-latest` by calling the same script. It is triggered by `workflow_call` from the umbrella release workflow, and by `workflow_dispatch` for manual runs.

Artifacts are uploaded as `desktop-windows-<arch>` and named:

```
LightNotes-<version>-windows-x86_64.msi
LightNotes-<version>-windows-x86_64-setup.exe
```

## Linux

### Building locally

```bash
make app-linux-bundle
```

That calls `scripts/linux-bundle.sh`, which runs `dx bundle --package-types appimage --package-types deb --package-types rpm` and renames the results to our release naming convention. Output lands in `dist/linux/`. It has to run on Linux.

The script is driven by environment variables, so CI and a local build take the same path:

| Variable | Default | Purpose |
| --- | --- | --- |
| `VERSION` | `dev` | Stamped into the artifact filenames |
| `ARCH` | host arch | `x86_64` or `aarch64`; also normalizes `amd64`/`arm64` |
| `TARGET` | host target | Rust target triple passed to `dx bundle` |
| `OUT_DIR` | `dist/linux` | Where the artifacts are written |

### System dependencies

The webview is WebKitGTK, which needs development headers present at build time:

```bash
sudo apt-get install -y --no-install-recommends \
  build-essential ca-certificates curl file git pkg-config wget \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libxdo-dev libssl-dev
```

Nothing else is needed. `dx` downloads `linuxdeploy` itself for the AppImage, and builds the RPM through a Rust crate rather than shelling out to `rpmbuild`. The AppImage step runs `linuxdeploy` in extract-and-run mode, so **FUSE is not required** — which is what lets the whole thing work inside a container.

### glibc floor

CI builds inside an `ubuntu:22.04` container. Binaries link against the glibc they were built on and cannot run on anything older, so building directly on the host would tie our floor to whatever the runner happens to be.

22.04 gives a **glibc 2.35** floor, which covers Ubuntu 22.04+, Debian 12+, and current Fedora. Pinning it in a container also means the floor is a deliberate choice recorded in the workflow, rather than a side effect of the host image.

### Desktop integration

`dx` generates the freedesktop `.desktop` entry and installs icons into `/usr/share/icons/hicolor/<size>x<size>/apps/`, picking the directory from each PNG's actual dimensions. That's why `icons/` holds a set at 32, 64, 128, 256 and 512 rather than only the 1024px master — a lone oversized icon lands in `hicolor/1024x1024`, which many launchers never look in.

`category = "Productivity"` in `Dioxus.toml` becomes `Categories=Office;` in the `.desktop` file, and `short_description` becomes `Comment=`.

The `.deb` declares its runtime dependencies (`libwebkit2gtk-4.1-0`, `libgtk-3-0`, `libayatana-appindicator3-1`) through `[bundle.deb] depends`, so `apt` pulls them in instead of the app failing to start. There is no equivalent setting for the RPM — `dx` exposes no RPM settings block — so the `.rpm` ships without declared requires. On a normal desktop Fedora install those libraries are already present.

### CI

`.github/workflows/release-desktop-linux.yml` runs on the `[self-hosted, homelab]` runner, inside the 22.04 container, calling the same script. It is triggered by `workflow_call` from the umbrella release workflow, and by `workflow_dispatch` for manual runs.

The macOS and Windows workflows stay on GitHub-hosted runners — the homelab runner is Linux X64, so it cannot build them.

Artifacts are uploaded as `desktop-linux-<arch>` and named:

```
LightNotes-<version>-linux-x86_64.AppImage
LightNotes-<version>-linux-x86_64.deb
LightNotes-<version>-linux-x86_64.rpm
```
