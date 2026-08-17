# Android 16 External Research Notes

The implementation boundary is based on official public documentation.

1. Android 16 target behavior: https://developer.android.com/about/versions/16/behavior-changes-16. Apps targeting API 36 cannot rely on the old edge-to-edge opt-out; predictive back behavior is enabled and legacy `onBackPressed`/back-key interception is no longer the supported path. The target therefore uses a full-window native surface and does not claim legacy back interception.

2. Android 16 all-app behavior: https://developer.android.com/about/versions/16/behavior-changes-all. Android 16 changes job quotas and warns against non-SDK/ART internals. The target does not use hidden APIs or assume unrestricted background execution.

3. Android NDK graphics guidance: https://developer.android.com/ndk/guides/graphics/getting-started. Public NDK guidance describes NativeActivity/android_main as the native lifecycle bridge and supports public native rendering paths on Vulkan-capable devices. The current target uses a public `ANativeWindow` surface and NDK touch events; physical Vulkan/device acceptance remains separate.

4. Motorola developer options: https://en-us.support.motorola.com/app/answers/detail/a_id/160067/~/developer-options. Motorola documents enabling Developer options by tapping Build number seven times under About phone/System. This is only a developer installation/testing aid and is not a production permission or root path.
