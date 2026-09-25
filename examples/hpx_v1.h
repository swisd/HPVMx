#ifndef HPVMX_HPX_V1_H
#define HPVMX_HPX_V1_H
#include <stddef.h>
#include <stdint.h>

typedef struct HpxHostApi {
    uint32_t abi_version;
    void (*draw_text)(size_t x, size_t y, const uint8_t *utf8, size_t len, uint32_t color);
    void (*fill_rect)(size_t x, size_t y, size_t width, size_t height, uint32_t color);
    int64_t (*read_file)(const uint8_t *path, size_t path_len, uint8_t *out, size_t capacity);
    int32_t (*write_file)(const uint8_t *path, size_t path_len, const uint8_t *data, size_t len);
    void *(*allocate)(size_t size, size_t alignment);
    void (*deallocate)(void *ptr, size_t size, size_t alignment);
} HpxHostApi;
uint32_t hpx_step(const HpxHostApi *host, void *state);
void hpx_draw(const HpxHostApi *host, void *state, size_t x, size_t y);
void hpx_input(const HpxHostApi *host, void *state, uint32_t packed_key);
#endif
