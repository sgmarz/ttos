# boot.S
# bootloader for SoS
# Stephen Marz
# 8 February 2019

# Disable generation of compressed instructions.
.option norvc

# Define a .text.start section. The .text.start is put at the
# starting address so that the entry _start is put at the RISC-V
# address at the starting point
.section .text.start

# Execution starts here.
.global _start
_start:
	# Disable linker instruction relaxation for the `la` instruction below.
	# This disallows the assembler from assuming that `gp` is already initialized.
	# This causes the value stored in `gp` to be calculated from `pc`.
	# The job of the global pointer is to give the linker the ability to address
	# memory relative to GP instead of as an absolute address.
.option push
.option norelax
	la		gp, __global_pointer$
.option pop
	la		sp, _sp0
	csrr	a0, mhartid
	slli	t3, a0, 12
	sub		sp, sp, t3
	la		ra, 4f
	li		t0, 0b11 << 11
	csrw	mstatus, t0
	la		t1, kinit
	csrw	mepc, t1
	mret


4:
	wfi
	j		4b


