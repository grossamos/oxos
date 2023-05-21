ENTRY(_start)
 
SECTIONS
{
    . = 0x80000;
    __start = .;
    .text :
    {
        *(.text._start)
        *(.text.kernel_main)
        *(.text*)
        *(.text)
    }
    .rodata :
    {
        *(.rodata*)
        *(.rodata)
    }
    .data :
    {
        *(.data*)
        *(.data)
    }
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss*)
        *(.bss)
    }
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }
    __bss_end = .;
    . = 0x82fc0; /* Align first program with 0x83000 */
    __end = .;
}

