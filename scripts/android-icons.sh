#!/usr/bin/env bash
set -euo pipefail

ICONS_DIR="${1:?usage: android-icons.sh <icons-dir> <res-dir> [--clean]}"
RES_DIR="${2:?usage: android-icons.sh <icons-dir> <res-dir> [--clean]}"
MODE="${3:-sync}"

for src in "$ICONS_DIR"/*/; do
  name="$(basename "$src")"
  for file in "$src"*; do
    rm -f "$RES_DIR/$name/$(basename "$file")"
  done
done

if [ "$MODE" = "--clean" ]; then
  exit 0
fi

rm -f "$RES_DIR"/mipmap-*/ic_launcher.webp
rm -f "$RES_DIR"/mipmap-anydpi-v26/ic_launcher.xml
rm -f "$RES_DIR"/drawable/ic_launcher_background.xml
rm -f "$RES_DIR"/drawable-v24/ic_launcher_foreground.xml

for src in "$ICONS_DIR"/*/; do
  name="$(basename "$src")"
  mkdir -p "$RES_DIR/$name"
  cp -R "$src". "$RES_DIR/$name/"
  echo "synced $name"
done
