.text
# ------------------------------------------------------------
# submission begin
# ------------------------------------------------------------
# arguments:
#   a0: number to multiply
# returns:
#   a0: numpler multiplied by PI in 16.16 format
.global mul_pi_16_16
mul_pi_16_16:
# ------------------------------------------------------------
# your code start
# ------------------------------------------------------------
    mv s1, ra
    li a1, 0x3243F
    li a2, 16
    jal mul_n_fixed_point
    mv ra, s1
# ------------------------------------------------------------
    # return
    jalr zero, 0(ra)
# ------------------------------------------------------------
# your code end
# ------------------------------------------------------------

.global mul_n_fixed_point
# arguments:
#   a0: first number in (32-N).N format
#   a1: second number in (32-N).N format
#   a2: number of decimal places (N)
# returns:
#   a0: result multiplication in (32-N).N format
mul_n_fixed_point:
# ------------------------------------------------------------
# your code start
# ------------------------------------------------------------
    mul a0, a0, a1  # lower in a0
    mulhu a3, a0, a1 # upper in a3

    # calc shift amount (32 - n)
    li a4, 32
    sub a4, a4, a2  # a4: 32-n

    sll a3, a3, a4  # shift left upper by (32 - n) bits

    srl a0, a0, a2  # shift right lower by n bits

    or a0, a0, a3   # copy shifted upper into a0
# ------------------------------------------------------------
# your code end
# ------------------------------------------------------------
    # return
    jalr zero, 0(ra)
# ------------------------------------------------------------
# submission end
# ------------------------------------------------------------
