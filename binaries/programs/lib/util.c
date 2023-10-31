#include "util.h"

bool _llToStr(unsigned long long v, bool neg, char *buf, int n) {
    char *end = buf + n;
    char *c = buf;

    if (v == 0 && c < end) {
        *(c++) = '0';
    }

    while (v != 0 && c < end) {
        unsigned long long rem = v % 10;
        v = v / 10;
        *(c++) = '0' + rem;
    }

    if (neg && c < end) {
        *(c++) = '-';
    }

    if (c < end) {
        int len = c - buf;
        for (int i = 0, j = len - 1; i < j; i++, j--) {
            char tmp = buf[i];
            buf[i] = buf[j];
            buf[j] = tmp;
        }
        *(c++) = '\0';
        return true;
    } else {
        return false;
    }
}

bool llToStr(long long v, char *buf, int n) {
    bool neg = v < 0;
    return _llToStr(neg ? -v : v, neg, buf, n);
}

bool lToStr(long v, char *buf, int n) {
    return llToStr(v, buf, n);
}

bool iToStr(int v, char *buf, int n) {
    return llToStr(v, buf, n);
}

bool ullToStr(unsigned long long v, char *buf, int n) {
    return _llToStr(v, false, buf, n);
}

bool ulToStr(unsigned long v, char *buf, int n) {
    return ullToStr(v, buf, n);
}

bool uiToStr(unsigned int v, char *buf, int n) {
    return ullToStr(v, buf, n);
}

const char* strstr(const char *haystack, const char *needle) {
    unsigned int needle_len = strlen(needle);
    for (const char *h = haystack; *h != '\0'; h++) {
        bool found = true;
        for (unsigned int n = 0; n < needle_len; n++) {
            if (*(h + n) != needle[n]) {
                found = false;
                break;
            }
        }
        if (found) return h;
    }
    return 0;
}

unsigned int strlen(const char *s) {

    unsigned int len = 0;
    while (*s++ != '\0') {
        len++;
    }

    return len;
}