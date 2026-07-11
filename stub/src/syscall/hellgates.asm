extern pSyscallStub


BITS 64
DEFAULT REL

section .data
    ; Variable global para guardar el SSN
    wSystemCall dd 0

section .text
global HellsGate
global HellDescent

; void HellsGate(u16 ssn)
; RCX contiene el argumento
HellsGate:
    mov dword [wSystemCall], ecx
    ret

; NTSTATUS HellDescent(arg1..arg11)
; Rust pone arg5+ en stack
HellDescent:
    mov r10, rcx
    mov eax, dword [wSystemCall]
    
    mov r11, [rel pSyscallStub]
    jmp r11