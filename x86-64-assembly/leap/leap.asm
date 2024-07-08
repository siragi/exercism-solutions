section .text
global leap_year
leap_year:
    ; int leap_year(int year) : c-prototype
    ; rax           rdi       : registers associated acc to calling convention (AMD64 ABI Reference.)
prologue:    
    ; push rbp
    ; mov rbp, rsp

division_tests:
    mov rax, rdi
    
    test rax, 0b11 ; Check bottom 2 bits to see if it's divisible by 4 (test uses bitwise and)
    jnz false   ; not divisible by 4

    test rax, 0b1111
    jz true ; divisible again by 4 (meaning all multiples of 16 are leap years like 400)

    xor rdx, rdx
    mov rcx, 25
    div rcx     ; divides (rdx:rax) by 25    ( /4/25 = /100 ) 
                ; result stored in RAX := Quotient, RDX := Remainder
    test rdx,rdx
    jz false  ; still divisible by 4, but all multiples of 25 like 100 are no leap year
    

true:
    mov rax, 1
    jmp epilogue
false:
    xor rax, rax

epilogue:
    ; mov rsp, rbp 
    ; pop rbp
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
