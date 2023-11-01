#include "lib/print.h"

int mul_pi_16_16(int x);
int mul_n_fixed_point(int a, int b, unsigned int n);

void main() {

    int x = 1 << 16;

    printf("{ib} * {ib}\n", x, 0x3243F);

    int y = mul_pi_16_16(x);

    printf("{ib}\n", y);
}

// 0000 0000 0000 0000  0000 0000 0000 0000
