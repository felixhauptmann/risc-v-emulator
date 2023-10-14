.section .text
.global _start

_start:
    li x31, 1024            # x31 = 1024
    li x30, 1               # x30 = 1
    li x29, 2               # x29 = 2

    sra x28, x31, x30       # x28 = 512
    sra x27, x28, x30       # x27 = 256
    sra x26, x27, x29       # x26 = 64

    li x25, -1024           # x25 = -1024

    sra x24, x25, x30       # x24 = -512
    sra x23, x24, x30       # x23 = -256
    sra x22, x23, x29       # x22 = -64