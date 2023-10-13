#	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o add.elf add.S
#	riscv64-unknown-elf-objcopy -O binary add.elf add.bin
#	riscv64-unknown-elf-objdump -d -f add.elf
#	riscv64-unknown-elf-objdump -b binary -m riscv -D add.bin

test_sources := $(wildcard *.S)
tests := $(test_sources:.S=.bin)

%.elf: %.S
	riscv64-unknown-elf-gcc -march=rv64g -Wl,-Ttext=0x0 -nostdlib -o $@ $^

%.bin: %.elf
	riscv64-unknown-elf-objcopy -O binary $^ $@

all: $(tests)

.phony: clean
clean:
	rm $(tests)