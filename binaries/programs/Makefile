binary_sources := $(wildcard *.c)
binaries := $(binary_sources:.c=.bin)

lib_sources := $(wildcard lib/*.c)

arch := rv32i
abi := ilp32
optlvl := 3

%.o: %.c
	riscv64-unknown-elf-gcc -march=$(arch) -mabi=$(abi) -Wall -Werror -O$(optlvl) -g -nostdlib -ffreestanding -fanalyzer -c $^ -o $@

%.o: %.s
	riscv64-unknown-elf-as -march=$(arch) -mabi=$(abi) -g crt0.s -o crt0.o

.PRECIOUS: %.elf
%.elf: %.o crt0.o mulfixed.S $(lib_sources)
	riscv64-unknown-elf-gcc -march=$(arch) -mabi=$(abi) -Wall -Werror -O$(optlvl) -g -nostdlib -ffreestanding -fanalyzer -T linker.ld $^ -o $@ -lgcc

%.bin: %.elf
	riscv64-unknown-elf-objcopy -O binary $^ $@

.phony: rv32i
rv32i: arch := rv32i
rv32i: abi := ilp32
rv32i: all

.phony: rv32e
rv32e: arch := rv32e
rv32e: abi := ilp32e
rv32e: all

.phony: rv64i
rv64i: arch := rv64i
rv64i: abi := lp64
rv64i: all

.phony: rv32im
rv32im: arch := rv32im
rv32im: abi := ilp32
rv32im: all

.phony: all
all: clean $(binaries)

.phony: clean
clean:
	rm -f *.bin *.elf crt0.o