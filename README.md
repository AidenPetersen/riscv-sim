# RISCV-SIM
The goal of this project is to be a cycle accurate OOO RISC-V CPU Simulator. 

## Memory
This simulator will accept ELF files, and during initialization of running a program the elf will start at `0x00000000`. The stack pointer will be initialized to `0x40000000`, and the PC will be set to the start of the text segment. 

## Limitations
This simulator will not include:
- interrupts
- ability to boot linux
- heap segments
- limited memory (Can use full 32 bit memory space)
- virtual memory (No OS to manage pages)
    - So no TLB
- multiple cores
