ENTRY(_start)
 
SECTIONS
{
    . = 0x80000;
    __start = .;
    .text :
    {
        *(.text.kernel_main)
        *(.text*)
        *(.text)
    }
    /*. = ALIGN(4096);*/
    .rodata :
    {
        *(.rodata*)
        *(.rodata)
    }
    /*. = ALIGN(4096);*/
    .data :
    {
        *(.data*)
        *(.data)
    }
    /*. = ALIGN(4096);*/
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss*)
        *(.bss)
    }
    /*. = ALIGN(4096); */
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }
    /*. = ALIGN(4096);*/
    __bss_end = .;
    __end = .;
}

