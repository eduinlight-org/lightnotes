#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROJECT="${1:?usage: android-overlay.sh <generated-android-project>}"
JAVA_SRC="$ROOT/apps/mobile/android/java"
MANIFEST="$PROJECT/app/src/main/AndroidManifest.xml"

if [ ! -f "$MANIFEST" ]; then
  echo "android-overlay: no manifest at $MANIFEST" >&2
  exit 1
fi

mkdir -p "$PROJECT/app/src/main/java"
cp -R "$JAVA_SRC/." "$PROJECT/app/src/main/java/"

python3 - "$MANIFEST" <<'PY'
import sys
import xml.etree.ElementTree as ET

ANDROID = "http://schemas.android.com/apk/res/android"
ET.register_namespace("android", ANDROID)
name = f"{{{ANDROID}}}name"

path = sys.argv[1]
tree = ET.parse(path)
root = tree.getroot()

application = root.find("application")
if application is None:
    raise SystemExit("android-overlay: manifest has no <application>")

receiver = "dev.lightnotes.mobile.ReminderReceiver"
if any(element.get(name) == receiver for element in application.findall("receiver")):
    print(f"android-overlay: {path} already declares the receiver")
    raise SystemExit(0)

ET.SubElement(application, "receiver", {name: receiver, f"{{{ANDROID}}}exported": "false"})
tree.write(path, encoding="utf-8", xml_declaration=True)
print(f"android-overlay: declared the reminder receiver in {path}")
PY
