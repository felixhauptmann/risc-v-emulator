.section .init
.globl _start
_start:
    jal main
    .word 0xffffffff # halt cpu
