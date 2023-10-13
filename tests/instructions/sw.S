# comment: This test depends on ADDI and LW

.section .text
.global _start

_start:
    addi x31, x0, 1234  # x31 = 1234

    addi sp, sp, -16

    sw x31, 0(sp)
    sw x31, 4(sp)
    sw x31, 8(sp)
    sw x31, 12(sp)

    lw x30, 0(sp)       # x30 = 1234
    lw x29, 4(sp)       # x29 = 1234
    lw x28, 8(sp)       # x28 = 1234
    lw x27, 12(sp)      # x27 = 1234

  addi sp, sp, 16