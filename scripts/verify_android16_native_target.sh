#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest="$root/android/app/src/main/AndroidManifest.xml"
gradle="$root/android/app/build.gradle"
cmake="$root/android/app/src/main/cpp/CMakeLists.txt"
source="$root/android/app/src/main/cpp/main.cpp"
for path in "$manifest" "$gradle" "$cmake" "$source"; do
  test -s "$path" || { echo "missing-or-empty:$path" >&2; exit 2; }
done
grep -q "targetSdk 36" "$gradle" || { echo "target-sdk-not-36" >&2; exit 2; }
grep -q 'android.app.NativeActivity' "$manifest" || { echo "native-activity-missing" >&2; exit 2; }
grep -q 'android.app.lib_name' "$manifest" || { echo "native-library-metadata-missing" >&2; exit 2; }
grep -q 'ANativeWindow_lock' "$source" || { echo "native-window-path-missing" >&2; exit 2; }
grep -q 'AMotionEvent_getX' "$source" || { echo "touch-path-missing" >&2; exit 2; }
if grep -qE 'MANAGE_EXTERNAL_STORAGE|SYSTEM_ALERT_WINDOW|BIND_ACCESSIBILITY_SERVICE|DEVICE_OWNER|su[[:space:]]|/dev/mem' "$manifest" "$source"; then
  echo "privileged-or-root-surface-detected" >&2
  exit 2
fi
printf '%s\n' 'android16_native_contract=pass' 'target_sdk=36' 'activity=android.app.NativeActivity' 'surface=ANativeWindow' 'touch=AMotionEvent' 'privileged_permissions=none'
if command -v adb >/dev/null 2>&1; then
  devices=$(adb devices | awk 'NR>1 && $2 == "device" {count++} END {print count+0}')
  printf 'authorized_devices=%s\n' "$devices"
  if [ "$devices" -gt 0 ]; then
    adb shell getprop ro.build.version.sdk
    adb shell getprop ro.product.model
  fi
else
  printf '%s\n' 'authorized_devices=unavailable-adb-not-installed'
fi
