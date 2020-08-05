#!/bin/sh

cargo build --release
if [ $? -ne 0 ]; then
	echo "Cargo build failed."
	exit 1
fi
llvm-objcopy -O binary target/riscv64gc-unknown-none-elf/release/ttos /srv/tftp/ttos.bin
