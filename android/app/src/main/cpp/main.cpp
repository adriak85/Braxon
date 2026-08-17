#include <android/log.h>
#include <android/native_window.h>
#include <android/input.h>
#include <android_native_app_glue.h>
#include <cstdint>
#include <cstring>

namespace {
constexpr char kTag[] = "BraxonNative";

struct SurfaceState {
    ANativeWindow* window = nullptr;
    int32_t width = 0;
    int32_t height = 0;
    float touch_x = 0.0F;
    float touch_y = 0.0F;
    bool touched = false;
    uint32_t frame = 0;
};

void log_error(const char* message) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "%s", message);
}

void draw_surface(SurfaceState* state) {
    if (state == nullptr || state->window == nullptr) {
        return;
    }

    ANativeWindow_Buffer buffer{};
    if (ANativeWindow_lock(state->window, &buffer, nullptr) != 0) {
        log_error("ANativeWindow_lock failed; refusing to claim a rendered frame");
        return;
    }

    state->width = buffer.width;
    state->height = buffer.height;
    auto* pixels = static_cast<uint32_t*>(buffer.bits);
    const int32_t stride = buffer.stride;
    for (int32_t y = 0; y < buffer.height; ++y) {
        for (int32_t x = 0; x < buffer.width; ++x) {
            const uint8_t r = static_cast<uint8_t>((x + state->frame) & 0xFFU);
            const uint8_t g = static_cast<uint8_t>((y + (state->frame * 2U)) & 0xFFU);
            const uint8_t b = state->touched ? 0xD0U : 0x48U;
            pixels[y * stride + x] = 0xFF000000U | (static_cast<uint32_t>(r) << 16U) |
                                      (static_cast<uint32_t>(g) << 8U) | b;
        }
    }

    if (state->touched) {
        const int32_t cx = static_cast<int32_t>(state->touch_x);
        const int32_t cy = static_cast<int32_t>(state->touch_y);
        for (int32_t y = cy - 24; y <= cy + 24; ++y) {
            if (y < 0 || y >= buffer.height) continue;
            for (int32_t x = cx - 24; x <= cx + 24; ++x) {
                if (x < 0 || x >= buffer.width) continue;
                const int32_t dx = x - cx;
                const int32_t dy = y - cy;
                if ((dx * dx) + (dy * dy) <= 576) {
                    pixels[y * stride + x] = 0xFFFFFFFFU;
                }
            }
        }
    }

    if (ANativeWindow_unlockAndPost(state->window) != 0) {
        log_error("ANativeWindow_unlockAndPost failed; frame was not acknowledged");
        return;
    }
    ++state->frame;
}

void handle_command(android_app* app, int32_t command) {
    auto* state = static_cast<SurfaceState*>(app->userData);
    if (state == nullptr) return;
    switch (command) {
        case APP_CMD_INIT_WINDOW:
            state->window = app->window;
            draw_surface(state);
            break;
        case APP_CMD_TERM_WINDOW:
            state->window = nullptr;
            break;
        case APP_CMD_GAINED_FOCUS:
            if (state->window != nullptr) draw_surface(state);
            break;
        default:
            break;
    }
}

int32_t handle_input(android_app* app, AInputEvent* event) {
    auto* state = static_cast<SurfaceState*>(app->userData);
    if (state == nullptr || AInputEvent_getType(event) != AINPUT_EVENT_TYPE_MOTION) return 0;
    const int32_t action = AMotionEvent_getAction(event) & AMOTION_EVENT_ACTION_MASK;
    if (action == AMOTION_EVENT_ACTION_DOWN || action == AMOTION_EVENT_ACTION_MOVE) {
        state->touch_x = AMotionEvent_getX(event, 0);
        state->touch_y = AMotionEvent_getY(event, 0);
        state->touched = true;
        draw_surface(state);
        return 1;
    }
    if (action == AMOTION_EVENT_ACTION_UP || action == AMOTION_EVENT_ACTION_CANCEL) {
        state->touched = false;
        draw_surface(state);
        return 1;
    }
    return 0;
}
}  // namespace

void android_main(android_app* app) {
    app_dummy();
    SurfaceState state{};
    app->userData = &state;
    app->onAppCmd = handle_command;
    app->onInputEvent = handle_input;

    while (true) {
        int events = 0;
        android_poll_source* source = nullptr;
        const int timeout = (state.window == nullptr) ? -1 : 16;
        const int result = ALooper_pollOnce(timeout, nullptr, &events, reinterpret_cast<void**>(&source));
        if (result >= 0 && source != nullptr) source->process(app, source);
        if (app->destroyRequested != 0) break;
        if (state.window != nullptr && result == ALOOPER_POLL_TIMEOUT) draw_surface(&state);
    }
}
