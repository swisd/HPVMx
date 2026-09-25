#include "hpx_v1.h"
#define COUNTER_PATH "\\counter.txt"

typedef struct {
    uint64_t frames;
    uint32_t color;
    uint32_t notice; /* 0 none, 1 saved, 2 loaded, 3 error, 4 network ready */
} CounterState;

static size_t format_u64(uint64_t value, uint8_t *out) {
    uint8_t reversed[20];
    size_t count = 0;
    do {
        reversed[count++] = (uint8_t)('0' + value % 10);
        value /= 10;
    } while (value && count < sizeof(reversed));

    for (size_t i = 0; i < count; ++i) out[i] = reversed[count - i - 1];
    return count;
}

static void save_counter(const HpxHostApi *host, CounterState *state) {
    uint8_t bytes[20];
    uint8_t digits[20];
    size_t length = format_u64(state->frames, digits);
    for (size_t i = 0; i < sizeof(bytes); ++i) bytes[i] = '0';
    for (size_t i = 0; i < length; ++i) bytes[sizeof(bytes) - length + i] = digits[i];
    state->notice = host->write_file((const uint8_t *)COUNTER_PATH,
                                     sizeof(COUNTER_PATH) - 1, bytes, sizeof(bytes)) == 0 ? 1 : 3;
}

static void load_counter(const HpxHostApi *host, CounterState *state) {
    const uint8_t *path = (const uint8_t *)COUNTER_PATH;
    int64_t length = host->read_file(path, sizeof(COUNTER_PATH) - 1, 0, 0);
    if (length != 20) { state->notice = 3; return; }

    uint8_t *bytes = (uint8_t *)host->allocate((size_t)length, 1);
    if (!bytes) { state->notice = 3; return; }
    int64_t actual = host->read_file(path, sizeof(COUNTER_PATH) - 1, bytes, (size_t)length);
    if (actual != length) {
        host->deallocate(bytes, (size_t)length, 1);
        state->notice = 3;
        return;
    }

    uint64_t value = 0;
    for (int64_t i = 0; i < length; ++i) {
        if (bytes[i] < '0' || bytes[i] > '9') {
            host->deallocate(bytes, (size_t)length, 1);
            state->notice = 3;
            return;
        }
        uint64_t digit = (uint64_t)(bytes[i] - '0');
        if (value > (UINT64_MAX - digit) / 10) {
            host->deallocate(bytes, (size_t)length, 1);
            state->notice = 3;
            return;
        }
        value = value * 10 + digit;
    }
    host->deallocate(bytes, (size_t)length, 1);
    state->frames = value;
    state->notice = 2;
}

uint32_t hpx_step(const HpxHostApi *host, void *opaque) {
    (void)host;
    CounterState *state = (CounterState *)opaque;
    ++state->frames;
    return 1;
}

void hpx_draw(const HpxHostApi *host, void *opaque, size_t x, size_t y) {
    CounterState *state = (CounterState *)opaque;
    host->fill_rect(x + 6, y + 6, 420, 90, 0x202830);
    static const uint8_t label[] = "Frames:";
    host->draw_text(x + 12, y + 12, label, sizeof(label) - 1, state->color ? state->color : 0xFFFFFF);
    uint8_t digits[20];
    size_t count = format_u64(state->frames, digits);
    host->draw_text(x + 76, y + 12, digits, count, 0x00FF88);

    static const uint8_t controls[] = "C: color S: save L: load B: beep N: network";
    host->draw_text(x + 12, y + 36, controls, sizeof(controls) - 1, 0xD0D0D0);
    static const uint8_t saved[] = "Saved counter";
    static const uint8_t loaded[] = "Loaded counter";
    static const uint8_t failed[] = "File operation failed";
    static const uint8_t network[] = "Network link is up";
    if (state->notice == 1) host->draw_text(x + 12, y + 56, saved, sizeof(saved) - 1, 0x80FF80);
    if (state->notice == 2) host->draw_text(x + 12, y + 56, loaded, sizeof(loaded) - 1, 0x80D0FF);
    if (state->notice == 3) host->draw_text(x + 12, y + 56, failed, sizeof(failed) - 1, 0xFF8080);
    if (state->notice == 4) host->draw_text(x + 12, y + 56, network, sizeof(network) - 1, 0x80FF80);

    if (host->abi_version >= HPX_HOST_API_VERSION) {
        HpxCpuInfo cpu;
        if (host->cpu_info(&cpu) == 0) {
            size_t vendor_len = 0;
            while (vendor_len < sizeof(cpu.vendor) && cpu.vendor[vendor_len]) ++vendor_len;
            host->draw_text(x + 12, y + 72, (const uint8_t *)cpu.vendor, vendor_len, 0xB0B0B0);
        }
    }
}

void hpx_input(const HpxHostApi *host, void *opaque, uint32_t packed_key) {
    CounterState *state = (CounterState *)opaque;
    if (packed_key & 0x10000) return;
    switch (packed_key & 0xFFFF) {
        case 'c': state->color = state->color == 0x00FFFF ? 0xFFFFFF : 0x00FFFF; break;
        case 's': save_counter(host, state); break;
        case 'l': load_counter(host, state); break;
        case 'b':
            if (host->abi_version >= HPX_HOST_API_VERSION) host->beep(660);
            break;
        case 'n':
            if (host->abi_version >= HPX_HOST_API_VERSION) {
                state->notice = host->network_initialize() == 0 && host->network_link_up() ? 4 : 3;
            }
            break;
        default: break;
    }
}
