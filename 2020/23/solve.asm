global _start

section .text
    NUM_CUPS equ 1000000
    NUM_ROUNDS equ 10000000

_start:
    mov dword [cups+0], 0
    mov rcx, 1
    xor rax, rax
    xor rbx, rbx
populate_initial:
    mov al, byte [initial_cups + rcx]
    mov bl, byte [initial_cups + rcx - 1]
    mov dword [cups + rbx*4], eax
    inc rcx
    cmp rcx, initial_cups_len
    jl populate_initial
    mov bl, byte [initial_cups + rcx - 1]
    inc rcx
    mov dword [cups + rbx*4], ecx
populate_initial2:
    mov rbx, rcx
    inc rcx
    mov dword [cups + rbx*4], ecx
    cmp rcx, NUM_CUPS
    jl populate_initial2
    xor rax, rax
    mov al, byte [initial_cups]
    mov rbx, NUM_CUPS
    mov dword [cups + rbx*4], eax

; Main loop
; Registers
; rcx: loop count
; r8d: current cup
; r9d: popped_cups[0]
; r10d; popped_cups[1]
; r11d; popped_cups[2]
;
; cur: eax
    xor r8, r8
    mov r8b, byte [initial_cups]
    mov rcx, NUM_ROUNDS
main_loop:
    mov r9d, [cups+r8d*4]
    mov r10d, [cups+r9d*4]
    mov r11d, [cups+r10d*4]
    mov r12d, [cups+r11d*4]
    mov [cups+r8d*4], r12d

    mov eax, r8d
find_label:
    sub eax, 1
    jz reset_label
cont:
    cmp eax, r9d
    je find_label
    cmp eax, r10d
    je find_label
    cmp eax, r11d
    je find_label
    mov ebx, [cups+eax*4]
    mov [cups+eax*4], r9d
    mov [cups+r11d*4], ebx

    mov r8d, [cups+r8d*4]

    sub rcx, 1
    jnz main_loop

    xor rax, rax
    xor rbx, rbx
    mov eax, [cups+4]
    mov ebx, [cups+eax*4]
    mul rbx

    call print_num
    
    mov rax, 60       ; exit(
    mov rdi, 0        ;   EXIT_SUCCESS
    syscall           ; );

reset_label:
    mov eax, NUM_CUPS
    jmp cont

print_num:
    push rdi
    push rdx
    push rsi
    push rax

    mov rdi, txtbuf
    call os_int_to_string
    mov byte [rdi], 10
    inc rdi
    mov byte [rdi], 0
    
    mov rdx, rdi
    sub rdx, txtbuf
    mov rax, 1        
    mov rdi, 1        
    mov rsi, txtbuf   
    syscall           
    
    pop rax
    pop rsi
    pop rdx
    pop rdi

    ret
; -----------------------------------------------------------------------------
; os_int_to_string -- Convert a binary interger into an string
;  IN:   RAX = binary integer
;   RDI = location to store string
; OUT:   RDI = points to end of string
;   All other registers preserved
; Min return value is 0 and max return value is 18446744073709551615 so your
; string needs to be able to store at least 21 characters (20 for the digits
; and 1 for the string terminator).
; Adapted from http://www.cs.usfca.edu/~cruse/cs210s09/rax2uint.s
os_int_to_string:
   push rdx
   push rcx
   push rbx
   push rax

   mov rbx, 10               ; base of the decimal system
   xor ecx, ecx               ; number of digits generated
os_int_to_string_next_divide:
   xor edx, edx               ; RAX extended to (RDX,RAX)
   div rbx                  ; divide by the number-base
   push rdx               ; save remainder on the stack
   inc rcx                  ; and count this remainder
   cmp rax, 0               ; was the quotient zero?
   jne os_int_to_string_next_divide      ; no, do another division

os_int_to_string_next_digit:
   pop rax                  ; else pop recent remainder
   add al, '0'               ; and convert to a numeral
   stosb                  ; store to memory-buffer
   loop os_int_to_string_next_digit      ; again for other remainders
   xor al, al
   stosb                  ; Store the null terminator at the end of the string

   pop rax
   pop rbx
   pop rcx
   pop rdx
   ret

section .rodata
    msg: db "Hello, world!", 10
    msglen: equ $ - msg
    initial_cups: db 8, 5, 3, 1, 9, 2, 6, 4, 7
    initial_cups_len: equ $ - initial_cups

section .bss
    txtbuf: resb 20
    cups: resd NUM_CUPS + 1


