#include "emulatorintrinsics.h"

void __hlt() {
   asm (".word 0xffffffff");
}

void __putchar(const char c) {
    register const char str_reg asm ("a0") = c;
    asm (".word 0xfffffffd" : : "r"(str_reg));
}