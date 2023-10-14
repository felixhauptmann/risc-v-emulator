.section .text
.global _start

_start:
    li x31, 1024        # x31 = 1024
    add x30, x31, x31   # x30 = 2048
    add x29, x30, x30   # x29 = 4096
    add x28, x29, x29   # x28 = 8192