#include <HPVMx>

uint32_t hpx_step(const void *host, void *state) {
    return 1;
}

void hpx_draw(const void *host, void *state, size_t x, size_t y) {
    //hpx_ui_clear(0x101820);
    hpx_ui_fill_rect(x + 12, y + 12, hpx_pack_dimensions(40, 120), 0x3070D0);
}
