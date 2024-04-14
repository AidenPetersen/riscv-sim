#!/bin/sh
for f in src/*.c; do
	name=$(basename $f .c)
	riscv32-none-elf-gcc -O0 -march=rv32id $f -c -o obj/$name.o
	riscv32-none-elf-ld -o elf/$name.elf obj/$name.o
done
