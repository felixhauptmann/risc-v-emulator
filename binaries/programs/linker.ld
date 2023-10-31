MEMORY
{
    ram : ORIGIN = 0x50000000, LENGTH = 0x8000000
}
SECTIONS
{
    .init : { *(.init*) } > ram
    .text : { *(.text*) } > ram
    .bss : { *(.bss*) } > ram
}