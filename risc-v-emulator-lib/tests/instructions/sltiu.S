.section .text
.global _start

_start:
    li x31, 1234            # x31 = 1234
    li x30, -1234           # x30 = -1234

    sltiu x29, x31, 1234    # x29 = 0
    sltiu x28, x31, -1234   # x28 = 1
    sltiu x27, x31, 0       # x27 = 0
    sltiu x26, x31, 1235    # x26 = 1
    sltiu x25, x31, 1233    # x25 = 0

    sltiu x24, x30, 1234    # x24 = 0
    sltiu x23, x30, -1234   # x23 = 0
    sltiu x22, x30, 0       # x22 = 0
    sltiu x21, x30, -1235   # x21 = 0
    sltiu x20, x30, -1233   # x20 = 1