#!/bin/sh

cargo clean > /dev/null 2>&1
cargo build --release > /dev/null 2>&1
if [ $? -ne 0 ]; then
	echo "Cargo build failed."
	exit 1
fi
llvm-objcopy -O binary target/riscv64gc-unknown-none-elf/release/ttos /srv/tftp/ttos.bin > /dev/null 2>&1
if [ $? -ne 0 ]; then
	echo "Unable to objcopy."
	exit 2
fi
exit 0
