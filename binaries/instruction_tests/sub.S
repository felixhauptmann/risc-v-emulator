.section .text
.global _start

_start:
    li x31, 1024        # x31 = 1024

    sub x30, x31, x31   # x30 = 0
    sub x29, x30, x31   # x29 = -1024
    sub x28, x29, x31   # x28 = -2048