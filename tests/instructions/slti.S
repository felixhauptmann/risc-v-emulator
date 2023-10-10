# comment: This test depends on ADDI

.section .text
.global _start

_start:
  addi x31, x0, 1234    # x31 = 1234
  addi x30, x0, -1234   # x30 = -1234

  slti x29, x31, 1234   # x29 = 0
  slti x28, x31, -1234  # x28 = 0
  slti x27, x31, 0      # x27 = 0
  slti x26, x31, 1235   # x26 = 1
  slti x25, x31, 1233   # x25 = 0

  slti x24, x30, 1234   # x24 = 1
  slti x23, x30, -1234  # x23 = 0
  slti x22, x30, 0      # x22 = 1
  slti x21, x30, -1235  # x21 = 0
  slti x20, x30, -1233  # x20 = 1