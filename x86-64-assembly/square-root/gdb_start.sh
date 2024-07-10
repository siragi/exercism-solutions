#! /usr/bin/env bash

nasm -g -F dwarf -f elf64 -o $1.o $1.asm
nasm -g -F dwarf -f elf64 -o test.o test.asm
ld -g -o test $1.o test.o
gdb test -ex 'break _start' -ex 'run' -ex 'layout asm'