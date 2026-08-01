#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/apps/mobile/Dioxus.toml"
ICONS="$ROOT/apps/mobile/icons/android"
PROJECT="$ROOT/target/dx/light-notes-mobile/release/android/app"
OUTPUTS="$PROJECT/app/build/outputs"

"$ROOT/scripts/android-icons.sh" "$ICONS" "$PROJECT/app/src/main/res" --clean

dx bundle --package light-notes-mobile --platform android --release --package-types apk --package-types aab

"$ROOT/scripts/android-icons.sh" "$ICONS" "$PROJECT/app/src/main/res"

if grep -q '^\[bundle\.android\]' "$MANIFEST"; then
  APK_TASK="assembleRelease"
else
  APK_TASK="assembleDebug"
fi

rm -f "$OUTPUTS"/bundle/release/*.aab

cd "$PROJECT"
./gradlew "$APK_TASK" bundleRelease

find "$OUTPUTS" -name "*.apk" -o -name "*.aab"
