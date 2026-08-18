# Braxon Android Entrance Split Bundles

These scripts package a large entrance payload into deterministic, independently hashed parts and reconstruct it only after validating every part and the final payload digest.

The Android entrance is intentionally a thin bridge. Its declared compatibility metadata is Android API 35 and API 36, corresponding to the requested Android 16 and Android 17 targets. The entrance does not replace the NSQ runtime: NSQ remains the runtime authority for semantic intent, initiative-cluster activation, JIT windows, and reconciliation.

## Create a split bundle

```sh
bash tools/android/package_entrance.sh path/to/payload-or-directory dist/braxon-android 64
```

For a directory, the script creates a deterministic tar payload before splitting it. For an APK or other file, it splits the file directly. The output contains `manifest.json` and ordered files under `parts/`.

## Reconstruct and verify

```sh
bash tools/android/bootstrap_entrance.sh dist/braxon-android dist/braxon-entrance.apk
```

The bootstrapper checks the schema, part count, contiguous indices, declared byte lengths, each part’s SHA-256, the final byte length, and the final SHA-256. Missing, reordered, modified, or extra-unexpected parts do not silently pass.

To attempt installation after verification:

```sh
bash tools/android/bootstrap_entrance.sh dist/braxon-android dist/braxon-entrance.apk --install
```

Installation requires `adb` and a connected device. A verified AAB is not directly installable with `adb`; it must be distributed or installed through an appropriate bundle tool.

## Acceptance boundary

The scripts prove packaging and reconstruction integrity. They do not prove Moto G acceptance, Android lifecycle behavior, Vulkan/WGPU surface creation, thermal stability, permissions, or performance. Those require a real non-rooted Android 16/17 device test. The repository currently has no Android SDK/Gradle/ADB toolchain available in the sandbox, so these scripts do not claim that an APK has already been built or installed.
