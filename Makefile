CC=riscv64-unknown-elf-gcc
CXX=riscv64-unknown-elf-g++
QEMU=/opt/riscv64/bin/qemu-system-riscv64
LLC=llc
OBJCOPY=riscv64-unknown-elf-objcopy
LDS=src/lds/k210.lds
LIBS=
CFLAGS= -Wall -O0 -T$(LDS) -mabi=lp64d -march=rv64g
CFLAGS+=-ffreestanding -nostartfiles -static -mcmodel=medany
ASM=$(wildcard asm/*.S)
ALL_SRCS=$(wildcard src/*.cpp)
HEADERS=$(wildcard src/*.h)
OUTPUT_DIR=objs/
OUT=os
OUT_BIN=os.bin


all: $(OUT) $(OUT_BIN)


$(OUT): $(ASM) Makefile $(OUTPUT_S) $(ALL_SRCS) $(LDS) $(HEADERS)
	$(CC) $(CFLAGS) -o $@ $(ASM) $(ALL_SRCS) $(LIBS)

$(OUT_BIN): $(OUT)
	$(OBJCOPY) -O binary $(OUT) $(OUT_BIN)

upload: $(OUT_BIN)
	./kflash.py -p /dev/ttyUSB0 -B maixduino $(OUT_BIN)

rung: $(OUT)
	$(QEMU) -S -s -nographic $(QEMU_ARGS) -kernel $(OUT)

runcon: $(OUT)
	$(QEMU) -nographic $(QEMU_ARGS) -kernel $(OUT)

run: $(OUT)
	$(QEMU) $(QEMU_ARGS) -kernel $(OUT)


.PHONY: clean

clean:
	rm -f $(OUT) $(OUT_IMG) $(OUT_BIN) $(wildcard $(OUTPUT_DIR)os.*) $(OUTPUT_LIB)
