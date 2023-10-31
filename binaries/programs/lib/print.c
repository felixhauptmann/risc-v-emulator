#include "print.h"

#include <stdarg.h>
#include <stdbool.h>

#include "util.h"
#include "emulatorintrinsics.h"

void _printstr(const char *str) {
    while (*str != '\0') {
        __putchar(*str++);
    }
}

int _countDigits(unsigned long long n) {
    int count = 1;
    while (n != 0) {
        n = n / 10;
        count++;
    }
    return count;
}

void _print2l(long long v) {
    int len = _countDigits(v < 0 ? -v : v);

    while (true) {
        char buf[len];

        if (llToStr(v, buf, len)) {
            _printstr(buf);
            break;
        }

        len *= 2;
    }
}

void _printU2l(unsigned long long v) {
    int len = _countDigits(v);

    while (true) {
        char buf[len];

        if (ullToStr(v, buf, len)) {
            _printstr(buf);
            break;
        }

        len *= 2;
    }
}

void _printbin(unsigned long long v, unsigned int n) {
    for (unsigned int i = 1; i <= n; i++) {
        if ((v & (0x1 << (n - i))) == 0) {
            __putchar('0');
        } else {
            __putchar('1');
        }
    }
}

void printf(const char *fmt, ...) {
    const char *const types[12] = {
        "{ull}",    // 0
        "{ul}",     // 1
        "{ll}",     // 2
        "{l}",      // 3
        "{ui}",     // 4
        "{i}",      // 5
        "{s}",      // 6
        "{c}",      // 7
        "{llb}",
        "{lb}",
        "{ib}",
        "{cb}",
    };

    va_list args;
    va_start(args, fmt);

    for (const char *c = fmt; *c != '\0';) {
        int type = -1;

        for (int t = 0; t < (sizeof(types) / sizeof(types[0])); t++) {
            if (strstr(c, types[t]) == c) {
                type = t;
                break;
            }
        }

        c += (type == -1) ? 0 : strlen(types[type]);

        switch (type) {
            case -1: {
                __putchar(*c++);
                break;
            }
            case 0: {
                unsigned long long ull = va_arg(args, unsigned long long);
                _printU2l(ull);
                break;
            }
            case 1: {
                unsigned long ul = va_arg(args, unsigned long);
                _printU2l(ul);
                break;
            }
            case 2: {
                long long ll = va_arg(args, long long);
                _print2l(ll);
                break;
            }
            case 3: {
                long l = va_arg(args, long);
                _print2l(l);
                break;
            }
            case 4: {
                unsigned int ui = va_arg(args, unsigned int);
                _printU2l(ui);
                break;
            }
            case 5: {
                int i = va_arg(args, int);
                _print2l(i);
                break;
            }
            case 6: {
                char* s = va_arg(args, char*);
                _printstr(s);
                break;
            }
            case 7: {
                __putchar(va_arg(args, int));
                break;
            }
            case 8: {
                unsigned long long ull = va_arg(args, unsigned long long);
                _printbin(ull, sizeof(unsigned long long) * 8);
                break;
            }
            case 9: {
                unsigned long ul = va_arg(args, unsigned long);
                _printbin(ul, sizeof(unsigned long) * 8);
                break;
            }
            case 10: {
                unsigned int ui = va_arg(args, unsigned int);
                _printbin(ui, sizeof(unsigned int) * 8);
                break;
            }
            case 11: {
                unsigned char uc = va_arg(args, int);
                _printbin(uc, sizeof(unsigned char) * 8);
                break;
            }
        }
    }

    va_end(args);
}