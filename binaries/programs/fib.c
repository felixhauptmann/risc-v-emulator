unsigned long long fib(unsigned long long n, unsigned long long* mem);

static const int N = 94; // fib(94) is the largest number fitting inside u64

int main() {
    unsigned long long res[N];
    for (int i = 0; i < N; i++) {
        res[i] = -1;
    }

    for (int i = 0; i < N; i++) {
        res[i] = fib(i, res);
    }

    return res[N - 1];
}

unsigned long long fib(unsigned long long n, unsigned long long* mem) {
    if (mem[n] != -1)
        return mem[n];

    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1, mem) + fib(n-2, mem));
}