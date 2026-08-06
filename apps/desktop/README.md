# Desktop

The desktop crate is the entrypoint for the LightNotes desktop app. It renders the shared `app` crate through a webview and ships as a native application on macOS, Windows and Linux.

```
desktop/
├─ assets/              # Desktop specific assets (main.css, tailwind.css)
├─ icons/               # Application icons used by the bundler, 32px through 1024px
├─ src/
│  └─ main.rs           # Entrypoint: mounts app::Route behind the desktop renderer
├─ Dioxus.toml          # Bundle metadata (identifier, publisher, icons, per-platform settings)
└─ Cargo.toml
```

## Development

```bash
make app-desktop-dev
```

`API_BASE_URL` is read at **compile time** (`option_env!`), so it has to be set for the build, not for the run. `make` loads it from `.env`; copy `.env.dist` to `.env` to get the local default of `http://localhost:4000`.

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
