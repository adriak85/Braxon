# Braxon Android 16 Native Target

This module is the **normal, unrooted Android target** for the Moto device. It uses `android.app.NativeActivity` and a public NDK `ANativeWindow` surface. The UI path is native C++ with direct touch-event handling; it does not use Compose, a Java widget hierarchy, a browser, an accessibility overlay, private APIs, root, or privileged device ownership.

Android 16 is API level 36. The module targets SDK 36, keeps orientation unrestricted, enables predictive-back compatibility, and uses an edge-to-edge-compatible theme. Android 16 disables the old edge-to-edge opt-out for applications targeting API 36, so the surface must be laid out without assuming inset-free legacy window behavior [1]. Android’s public NDK documentation describes `NativeActivity`/`android_main` as the bridge from Android lifecycle events into native code and documents Vulkan/native rendering as a supported path [2].

The current proof surface uses `ANativeWindow_lock` and `ANativeWindow_unlockAndPost` so it remains buildable as a minimal public-NDK surface. A later Vulkan renderer can replace the framebuffer fill behind the same lifecycle and input boundary. The target does not claim direct access to CPU registers, the MMU, Android compositor internals, or physical device memory. The NSQ/Piston virtual-extension contract remains software-owned.

## Ethical no-root boundary

A normal Android application may own its activity window and receive touch input delivered to that window. It cannot ethically or technically bypass Android’s application sandbox, map arbitrary physical memory, replace the system compositor, or keep a process alive indefinitely outside Android lifecycle and battery policy. The app therefore uses foreground-visible execution for interactive bus work and bounded, user-visible persistence for longer tasks. It does not request `MANAGE_EXTERNAL_STORAGE`, accessibility-service control, device-owner status, root, or hidden permissions.

The Moto developer-options path is only needed for developer testing and installation: Motorola documents enabling Developer options by tapping **Build number** seven times under Settings > About phone/System > About phone [3]. No production operation depends on developer options.

## Device acceptance

A real acceptance run requires a physical Android 16 Moto device with USB debugging authorized or an equivalent user-controlled installation path. The host repository contains the native source and static contract checks, but no physical device was connected during this audit (`adb` was unavailable in the validation environment). The acceptance record must include the device API level, ABI, package install result, first-frame result, touch event result, back-navigation result, rotation/resume result, and clean uninstall result.

## References

[1]: https://developer.android.com/about/versions/16/behavior-changes-16 "Android 16 behavior changes for apps targeting API 36"
[2]: https://developer.android.com/ndk/guides/graphics/getting-started "Android NDK Vulkan and NativeActivity guidance"
[3]: https://en-us.support.motorola.com/app/answers/detail/a_id/160067/~/developer-options "Motorola developer options"
