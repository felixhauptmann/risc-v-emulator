#include "lib/print.h"

static const int N = 10000;

int nthPrime(int n, void *data, void (*callback)(int, void *)) {
    int numList[n];
    int i, len = 0, num = 2;

    numList[len++] = 2;
    callback(2, data);
    while (len < n) {
        for (i = 0; i < len; i++) {
            if (num % numList[i] == 0)
                break;
            else if (numList[i] * numList[i] > num) {
                numList[len++] = num;
                callback(num, data);
                break;
            }
        }
        num++;
    }
    return numList[n - 1];
}

void callback(int num, void *data) {
    printf("{i}\r", num);
}

void main() {
    printf("test");
    printf("Calculating Prime({i}):\n", N);
    printf("{i}\n", nthPrime(N, 0, callback));
}

