#pragma once

void __hlt();

#define __dbg() asm (".word 0xfffffffe")

void __putchar(char c);