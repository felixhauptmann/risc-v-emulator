.section .text
.global _start

_start:
    li x31, 1
    li x30, 1

    jal test

    li x31, 2   # x31 = 2

    jal end

test:
    li x30, 2   # x30 = 2
    ret

end:
