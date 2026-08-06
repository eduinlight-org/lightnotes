#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

VERSION="${VERSION:-dev}"
TARGET="${TARGET:-}"
OUT_DIR="${OUT_DIR:-$ROOT/dist/macos}"

case "${ARCH:-$(uname -m)}" in
  arm64 | aarch64) ARCH="aarch64" ;;
  x86_64 | amd64) ARCH="x86_64" ;;
  *) echo "unsupported architecture: ${ARCH:-$(uname -m)}" >&2; exit 1 ;;
esac

BASE="LightNotes-${VERSION}-macos-${ARCH}"

for var in APPLE_CERTIFICATE APPLE_CERTIFICATE_PASSWORD APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID; do
  if [ -z "${!var:-}" ]; then
    unset "$var"
  fi
done

if [ -z "${APPLE_CERTIFICATE:-}" ]; then
  echo "::warning::APPLE_CERTIFICATE is not set, producing an unsigned build"
elif [ -z "${APPLE_ID:-}" ]; then
  echo "::warning::APPLE_ID is not set, the build will be signed but not notarized"
fi

mkdir -p "$OUT_DIR"
rm -rf "$OUT_DIR"/*.app "$OUT_DIR"/*.dmg "$OUT_DIR"/*.app.zip

bundle_args=(
  --package desktop
  --platform macos
  --release
  --package-types macos
  --package-types dmg
  --out-dir "$OUT_DIR"
)
if [ -n "$TARGET" ]; then
  bundle_args+=(--target "$TARGET")
fi

dx bundle "${bundle_args[@]}"

APP="$(find "$OUT_DIR" -maxdepth 1 -name '*.app' | head -n 1)"
DMG="$(find "$OUT_DIR" -maxdepth 1 -name '*.dmg' | head -n 1)"

if [ -z "$APP" ] || [ -z "$DMG" ]; then
  echo "dx bundle did not produce both a .app and a .dmg in $OUT_DIR" >&2
  ls -la "$OUT_DIR" >&2
  exit 1
fi

mv "$DMG" "$OUT_DIR/${BASE}.dmg"
ditto -c -k --sequesterRsrc --keepParent "$APP" "$OUT_DIR/${BASE}.app.zip"

ls -la "$OUT_DIR"
