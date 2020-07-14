#!/bin/sh

cargo build --release
if [ $? -ne 0 ]; then
	echo "Cargo build failed."
	exit 1
fi
llvm-objcopy -O binary ttos ttos.bin
if [ $? -ne 0 ]; then
	echo "Objcopy failed."
	exit 1
fi
python kflash.py -p /dev/ttyUSB0 -B maixduino ttos.bin
