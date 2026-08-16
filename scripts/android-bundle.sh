#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/apps/mobile/Dioxus.toml"
ICONS="$ROOT/apps/mobile/icons/android"
PROJECT="$ROOT/target/dx/light-notes/release/android/app"
OUTPUTS="$PROJECT/app/build/outputs"

VERSION="${VERSION:-dev}"
ARCH="${ARCH:-arm64}"
OUT_DIR="${OUT_DIR:-$ROOT/dist/android}"

BASE="LightNotes-${VERSION}-android-${ARCH}"

mkdir -p "$OUT_DIR"
rm -f "$OUT_DIR"/*.apk "$OUT_DIR"/*.aab

"$ROOT/scripts/android-icons.sh" "$ICONS" "$PROJECT/app/src/main/res" --clean

dx bundle --package light-notes-mobile --platform android --release --package-types apk --package-types aab

"$ROOT/scripts/android-icons.sh" "$ICONS" "$PROJECT/app/src/main/res"
"$ROOT/scripts/android-overlay.sh" "$PROJECT"

if grep -q '^\[bundle\.android\]' "$MANIFEST"; then
  APK_TASK="assembleRelease"
else
  APK_TASK="assembleDebug"
  echo "android-bundle: $MANIFEST has no [bundle.android] signing block, the APK will be debug-signed" >&2
fi

rm -rf "$OUTPUTS/apk" "$OUTPUTS/bundle"

cd "$PROJECT"
./gradlew "$APK_TASK" bundleRelease

for ext in apk aab; do
  found="$(find "$OUTPUTS" -name "*.${ext}" | head -n 1)"
  if [ -z "$found" ]; then
    echo "gradle produced no .${ext} in $OUTPUTS" >&2
    find "$OUTPUTS" -type f >&2
    exit 1
  fi
  mv "$found" "$OUT_DIR/${BASE}.${ext}"
done

ls -la "$OUT_DIR"
