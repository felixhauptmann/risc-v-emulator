.section .text
.global _start

_start:
  addi x31, x0, 0       # x31 = 0
  addi x30, x31, 1024   # x30 = 1024
  addi x29, x30, 1024   # x29 = 2048
  addi x28, x29, -1024  # x28 = 1024
  addi x27, x0, -1      # x27 = -1
  addi x26, x27, 1      # x26 = 0