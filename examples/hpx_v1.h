#ifndef HPVMX_HPX_V1_H
#define HPVMX_HPX_V1_H
#include <stddef.h>
#include <stdint.h>

#define HPX_EXECUTABLE_ABI_VERSION 1
#define HPX_HOST_API_VERSION 2

typedef struct HpxCpuInfo HpxCpuInfo;
typedef struct HpxPciDeviceInfo HpxPciDeviceInfo;

typedef struct _HpxHostApi {
    uint32_t abi_version;
    void (*draw_text)(size_t x, size_t y, const uint8_t *utf8, size_t len, uint32_t color);
    void (*fill_rect)(size_t x, size_t y, size_t width, size_t height, uint32_t color);
    void (*draw_line)(size_t x0, size_t y0, size_t x1, size_t y1, uint32_t color);
    int64_t (*read_file)(const uint8_t *path, size_t path_len, uint8_t *out, size_t capacity);
    int32_t (*write_file)(const uint8_t *path, size_t path_len, const uint8_t *data, size_t len);
    void *(*allocate)(size_t size, size_t alignment);
    void (*deallocate)(void *ptr, size_t size, size_t alignment);
    /* Host API v2 extensions are appended so the original v1 field offsets stay fixed. */
    int32_t (*file_make_dir)(const uint8_t *path, size_t path_len);
    int32_t (*file_remove)(const uint8_t *path, size_t path_len);
    int32_t (*file_rename)(const uint8_t *from, size_t from_len,
                           const uint8_t *to, size_t to_len);
    int64_t (*current_directory)(uint8_t *out, size_t capacity);
    int32_t (*set_global_variable)(const uint8_t *key, size_t key_len,
                                   const uint8_t *value, size_t value_len);
    int64_t (*get_global_variable)(const uint8_t *key, size_t key_len,
                                   uint8_t *out, size_t capacity);
    int32_t (*cpu_info)(HpxCpuInfo *out);
    uint32_t (*pci_device_count)(void);
    int32_t (*pci_get_device)(uint32_t index, HpxPciDeviceInfo *out);
    uint32_t (*pci_read_u32)(uint8_t bus, uint8_t slot, uint8_t function, uint8_t offset);
    void (*pci_write_u32)(uint8_t bus, uint8_t slot, uint8_t function,
                          uint8_t offset, uint32_t value);
    int32_t (*network_initialize)(void);
    uint8_t (*network_link_up)(void);
    int32_t (*network_transmit)(const uint8_t *frame, size_t len);
    int64_t (*network_receive)(uint8_t *out, size_t capacity);
    void (*beep)(uint32_t frequency_hz);
    void (*play_tone)(uint32_t frequency_hz, uint64_t duration_ms);
    void (*mute)(void);
    void (*sleep_ms)(uint64_t milliseconds);
} HpxHostApi;

struct HpxCpuInfo {
    char vendor[13];
    char brand[49];
    uint32_t cores;
    uint32_t threads;
    uint32_t ap_count;
    /* Bits: 0=64-bit, 1=VMX, 2=SVM, 3=AVX2, 4=SSE4.2, 5=MP. */
    uint32_t feature_flags;
};

struct HpxPciDeviceInfo {
    uint8_t bus, device, function;
    uint8_t class_id, subclass_id, interface_id, revision_id, reserved;
    uint16_t vendor_id, device_id;
};
uint32_t hpx_step(const HpxHostApi *host, void *state);
void hpx_draw(const HpxHostApi *host, void *state, size_t x, size_t y);
void hpx_input(const HpxHostApi *host, void *state, uint32_t packed_key);
#endif
