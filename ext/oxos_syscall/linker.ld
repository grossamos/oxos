ENTRY(_start)
 
SECTIONS
{
    . = 0x83000;
    __start = .;
    .text :
    {
        KEEP(*(.text._start))
        *(.text)
    }
    .rodata :
    {
        *(.rodata*)
        *(.rodata)
    }
    .data :
    {
        *(.data)
    }
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }
    __bss_end = .;
    __end = .;
}
