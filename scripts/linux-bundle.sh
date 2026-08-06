#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

VERSION="${VERSION:-dev}"
TARGET="${TARGET:-}"
OUT_DIR="${OUT_DIR:-$ROOT/dist/linux}"

case "${ARCH:-$(uname -m)}" in
  x86_64 | amd64) ARCH="x86_64" ;;
  aarch64 | arm64) ARCH="aarch64" ;;
  *) echo "unsupported architecture: ${ARCH:-$(uname -m)}" >&2; exit 1 ;;
esac

BASE="LightNotes-${VERSION}-linux-${ARCH}"

mkdir -p "$OUT_DIR"
rm -f "$OUT_DIR"/*.AppImage "$OUT_DIR"/*.deb "$OUT_DIR"/*.rpm

bundle_args=(
  --package desktop
  --platform linux
  --release
  --package-types appimage
  --package-types deb
  --package-types rpm
  --out-dir "$OUT_DIR"
)
if [ -n "$TARGET" ]; then
  bundle_args+=(--target "$TARGET")
fi

dx bundle "${bundle_args[@]}"

for ext in AppImage deb rpm; do
  found="$(find "$OUT_DIR" -maxdepth 1 -name "*.${ext}" | head -n 1)"
  if [ -z "$found" ]; then
    echo "dx bundle produced no .${ext} in $OUT_DIR" >&2
    ls -la "$OUT_DIR" >&2
    exit 1
  fi
  mv "$found" "$OUT_DIR/${BASE}.${ext}"
done

chmod +x "$OUT_DIR/${BASE}.AppImage"

ls -la "$OUT_DIR"
