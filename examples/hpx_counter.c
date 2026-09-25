#include "hpx_v1.h"
typedef struct { uint64_t frames; uint32_t color; } CounterState;

uint32_t hpx_step(const HpxHostApi *host, void *opaque) {
    (void)host;
    CounterState *state = (CounterState *)opaque;
    ++state->frames;
    return 1;
}

void hpx_draw(const HpxHostApi *host, void *opaque, size_t x, size_t y) {
    CounterState *state = (CounterState *)opaque;
    static const uint8_t label[] = "Frames:";
    host->draw_text(x + 12, y + 12, label, sizeof(label) - 1, state->color ? state->color : 0xFFFFFF);
    char digits[21]; size_t count = 0; uint64_t n = state->frames;
    do { digits[count++] = (char)('0' + n % 10); n /= 10; } while (n && count < sizeof(digits));
    for (size_t i = 0; i < count / 2; ++i) { char t = digits[i]; digits[i] = digits[count - 1 - i]; digits[count - 1 - i] = t; }
    host->draw_text(x + 76, y + 12, (const uint8_t *)digits, count, 0x00FF88);
}

void hpx_input(const HpxHostApi *host, void *opaque, uint32_t packed_key) {
    (void)host;
    CounterState *state = (CounterState *)opaque;
    state->color = (packed_key & 0xFFFF) == 'c' ? 0x00FFFF : 0xFFFFFF;
}
