int fib(unsigned int n, unsigned int* mem);

static const int N = 48; // fib(48) is the largest number fitting inside u32

int main() {

    unsigned int res[N];
    for (int i = 0; i < N; i++) {
        res[i] = -1;
    }

    for (int i = 0; i < N; i++) {
        res[i] = fib(i, res);
    }

    return res[N - 1];
}

int fib(unsigned int n, unsigned int* mem) {
    if (mem[n] != -1)
        return mem[n];

    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1, mem) + fib(n-2, mem));
}