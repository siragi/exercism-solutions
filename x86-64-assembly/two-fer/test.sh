nasm -f elf64 -o $1.o $1.asm -g -F dwarf
nasm -f elf64 -o test.o test.asm -g -F dwarf
ld -o test test.o $1.o 