# riscv64-unknown-elf-gcc -march=rv32ifd -mabi=ilp32 -Wl,-Ttext=0x0 -nostdlib -o fib.elf fib.c
# riscv64-unknown-elf-objcopy -O binary add.elf add.bin

test_sources := $(wildcard *.c)
tests := $(test_sources:.c=.bin)

%.elf: %.c
	riscv64-unknown-elf-gcc -Os -fno-builtin -march=rv32ifd -mabi=ilp32 -Wl,-Ttext=0x80000000 -nostdlib -o $@ $^

%.bin: %.elf
	riscv64-unknown-elf-objcopy -O binary $^ $@

all: fib.bin

.phony: clean
clean:
	rm $(tests)