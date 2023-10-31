#include "lib/print.h"

static const int N = 94; // fib(94) is the largest number fitting inside u64

unsigned long long fib(unsigned long long n, unsigned long long* mem) {
    if (mem[n] != -1)
        return mem[n];

    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1, mem) + fib(n-2, mem));
}

void main() {
    unsigned long long res[N];
    for (int i = 0; i < N; i++) {
        res[i] = -1;
    }

    for (int i = 0; i < N; i++) {
        res[i] = fib(i, res);
    }

    for (int i = 0; i < N; i++) {
        printf("{ull}\n", res[i]); // TODO: investigate, this is buggy on rv64i
    }
}
