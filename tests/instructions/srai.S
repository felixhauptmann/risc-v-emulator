.section .text
.global _start

_start:
    li x31, 1024        # x31 = 1024

    srai x30, x31, 1    # x30 = 512
    srai x29, x30, 1    # x29 = 256
    srai x28, x29, 2    # x28 = 64

    li x27, -1024       # x27 = -1024

    srai x26, x27, 1    # x26 = -512
    srai x25, x26, 1    # x25 = -256
    srai x24, x25, 2    # x24 = -64