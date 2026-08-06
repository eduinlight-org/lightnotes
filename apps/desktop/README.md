# Desktop

The desktop crate is the entrypoint for the LightNotes desktop app. It renders the shared `app` crate through a webview and ships as a native application on macOS, Windows and Linux.

```
desktop/
├─ assets/              # Desktop specific assets (main.css, tailwind.css)
├─ icons/               # Application icon used by the bundler
├─ src/
│  └─ main.rs           # Entrypoint: mounts app::Route behind the desktop renderer
├─ Dioxus.toml          # Bundle metadata (identifier, publisher, icon, per-platform settings)
└─ Cargo.toml
```

## Development

```bash
make app-desktop-dev
```

`API_BASE_URL` is read at **compile time** (`option_env!`), so it has to be set for the build, not for the run. `make` loads it from `.env`; copy `.env.dist` to `.env` to get the local default of `http://localhost:4000`.

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
