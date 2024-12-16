/* memory.x
   Similar memory layout as the provided linker.ld for RISC-V */

MEMORY
{
  /* Define FLASH memory starting at 0x90000400, 512MB in size */
  FLASH : ORIGIN = 0x90000400, LENGTH = 512M

  /* Define RAM starting at 0x80000000, 128KB in size */
  RAM   : ORIGIN = 0x80000000, LENGTH = 128K

  /* Define PSRAM starting at 0xB0000000, 512MB in size */
  PSRAM : ORIGIN = 0xB0000000, LENGTH = 512M
}

/* Define stack size */
_STACK_SIZE = 1K;

/* Define memory regions to map the sections */
SECTIONS
{
  /* Code section placed in RAM */
  .text.init :
  {
    *(.text.init)
  } > RAM

  . = ALIGN(8);
  .text :
  {
    *(.text)
    . = ALIGN(8);
    *(.text.*)
  } > RAM

  . = ALIGN(8);
  .rodata :
  {
    *(.rodata)
    *(.rodata.*)
    *(.gnu.linkonce.r.*)
  } > RAM

  /* Initialized data sections */
  _la_sdata = LOADADDR(.sdata);
  .sdata :
  {
    *(.srodata.cst16) *(.srodata.cst8) *(.srodata.cst4) *(.srodata.cst2) *(.srodata*)
    *(.sdata .sdata.* .gnu.linkonce.s.*)
  } > RAM AT> RAM

  _la_data = LOADADDR(.data);
  .data :
  {
    *(.data)
    *(.data.*)
    *(.gnu.linkonce.d.*)
    *(.tdata.begin) *(.tdata) *(.tdata.end)
    *(.tbss) *(.tbss.end)
  } > RAM AT> RAM

  /* Uninitialized sections */
  .sbss :
  {
    *(.sbss)
    *(.sbss.*)
    *(.gnu.linkonce.sb.*)
  } > RAM

  .bss :
  {
    *(.bss)
    *(.bss.*)
    *(.gnu.linkonce.b.*)
    *(COMMON)
  } > RAM

  /* Stack definition */
  .stack :
  {
    . = ALIGN(8);
    _stack = .;
    _stack_end = . - _STACK_SIZE;
  } > RAM

  /* Heap definition */
  .heap :
  {
    _HEAP_SIZE = _stack_end - ORIGIN(RAM);
    _heap = _stack_end - _HEAP_SIZE;
  } > RAM
}
