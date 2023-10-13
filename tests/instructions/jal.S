.section .text
.global _start

_start:
    li x31, 1   # x31 = 1
    li x30, 1

    jal test

    li x31, 2

test:
    li x30, 2   # x30 = 2
