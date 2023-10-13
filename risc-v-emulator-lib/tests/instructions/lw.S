.section .text
.global _start

_start:
    lw x30, a   # x30 = 1234
    lw x29, a   # x29 = 1234
    lw x28, a   # x28 = 1234
    lw x27, a   # x27 = 1234

.section .data
a:
    .word 1234