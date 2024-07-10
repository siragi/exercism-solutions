section .text

global square_root
square_root:
    ; rdi: int (radicand)
    bsr ecx, edi    ; ecx is floored log2 of radicand (of most significant bit only). (between 0..31)
                    ; bsr (Bit Scan Reverse searches the source operand (second operand) for the most significant set bit (1 bit))
                    ; sqrt(x) = x^(1/2) = (2^log2(x))^(1/2) = 2^(log2(x)*1/2) = 2^(log2(x)/2)
    inc ecx         ; ecx = floor(log2(x)) + 1 
                    ; (this is a hack to reduce the avg error to the original sqrt(x) function (better model))
    shr ecx, 1      ; floor((floor(log2(x))+1)/2)
    mov eax, 1      
    shl eax, cl     ; 2^(floor((floor(log2(x))+1)/2)) our first estimate for sqrt(x)
                    ; ex. 25 = 0b11001; bsr=4 (first set bit on: 2^4) 1--shr (4(+1)/2)--shr 2--> 0b100 = 4

    mov edx, edi
    shr edx, cl     ; complementary estimate (upper half bits): ex. 25 = 0b11001 --shr (4(+1)/2)--shr 2--> 0b110.01 = 6

    add eax, edx
    shr eax, 1      ; Avg of both estimates (accuracy is enough for the current tests, would fail for 22 though :-)

    mov ecx, eax    ; save estimate

; loop:             ; loop is necessary for higher number like sqrt(2147395600) 
;     mov r8, rcx ; save current estimate
;     mov rax, rdi
;     xor rdx, rdx
;     div rcx     ;Divides unsigned the value in RDX:RAX registers (dividend) by the source operand (divisor) 
;                 ; and stores the result in the RDX:RAX registers. (RAX := Quotient, RDX := Remainder)
;     add rcx, rax    ; (estimate + (radicand / estimate))
;     shr rcx, 1      ; new estimate is the half

;     cmp r8,rcx
;     jne loop
    
;     mov rax, rcx
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
