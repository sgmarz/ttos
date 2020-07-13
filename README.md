# ttos
Teeny Tiny Operating System (ttos)


```
riscv64-unknown-elf-objcopy -O binary $(OUT) $(OUT_BIN)
./kflash.py -p /dev/ttyUSB0 -B maixduino $(OUT_BIN)
```
