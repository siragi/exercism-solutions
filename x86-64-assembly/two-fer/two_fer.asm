section .rodata
    ; "One for Bob, one for me."
    out1 db "One for ",0
    out1Len          equ     $-out1
    out2 db ", one for me.", 0
    out2Len          equ     $-out2
    you db "you", 0
section .text
global two_fer
default rel

two_fer:
    mov rdx, rdi    ; name into rdx
    test rdx, rdx
    lea r8, [you]
    cmovz rdx, r8   ; default if empty name
    mov rdi, rsi    ; buffer into rdi

appendOut1:    
    cld             ; Clears the direction flag. Registers will increment, reading forwards.
    mov rcx, out1Len
    lea rsi, [out1]
    rep movsb       ; moves from rsi to rdi rcx times (bytes)
    dec rdi
appendName:
    mov al, [rdx] 
    mov [rdi], al
    inc rdx
    inc rdi

    test al, al
    jnz appendName
    
appendOut2:
    dec rdi
    mov rcx, out2Len
    lea rsi, [out2]
    rep movsb
    ret


%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif