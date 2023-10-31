#pragma once

#include <stdbool.h>

bool llToStr(long long v, char *buf, int n);

bool lToStr(long v, char *buf, int n);

bool iToStr(int v, char *buf, int n);

bool ullToStr(unsigned long long v, char *buf, int n);

bool ulToStr(unsigned long v, char *buf, int n);

bool uiToStr(unsigned int v, char *buf, int n);

const char* strstr(const char *haystack, const char *needle);

unsigned int strlen(const char *s);