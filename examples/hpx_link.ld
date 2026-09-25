SECTIONS {
  . = 0;
  .text : { *(.text.hpx_step) *(.text.hpx_draw) *(.text.hpx_input) *(.text*) }
  .rodata : { *(.rodata*) }
  .got : { *(.got*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) *(COMMON) }
  /DISCARD/ : { *(.comment) *(.eh_frame*) *(.note*) }
}
