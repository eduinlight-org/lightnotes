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
import sys, xml.etree.ElementTree as ET

ANDROID = "http://schemas.android.com/apk/res/android"
ET.register_namespace("android", ANDROID)
name = f"{{{ANDROID}}}name"

path = sys.argv[1]
tree = ET.parse(path)
root = tree.getroot()

permissions = [
    "android.permission.POST_NOTIFICATIONS",
    "android.permission.SCHEDULE_EXACT_ALARM",
    "android.permission.USE_EXACT_ALARM",
    "android.permission.RECEIVE_BOOT_COMPLETED",
]

declared = {element.get(name) for element in root.findall("uses-permission")}
for permission in permissions:
    if permission not in declared:
        ET.SubElement(root, "uses-permission", {name: permission})

application = root.find("application")
if application is None:
    raise SystemExit("android-overlay: manifest has no <application>")

receiver_name = "dev.lightnotes.mobile.ReminderReceiver"
if not any(element.get(name) == receiver_name for element in application.findall("receiver")):
    ET.SubElement(
        application,
        "receiver",
        {name: receiver_name, f"{{{ANDROID}}}exported": "false"},
    )

tree.write(path, encoding="utf-8", xml_declaration=True)
print(f"android-overlay: patched {path}")
PY
