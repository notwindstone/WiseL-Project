format PE64 Console
entry start

include 'win64w.inc'

section '.data' data readable writeable
    wisel_v1 dd ?
    hStdOut dq ?
    hStdIn dq ?
    written dd ?
    readLen dd ?
    newline db 13,10,0
    newline_len dd 2
    readBuffer db 512 dup(?)
    tempWide dw 256 dup(?)
    numBuffer db 32 dup(?)
    bytesWritten dd ?
    charsRead dd ?
    buf1 db 512 dup(?)
    buf2 db 512 dup(?)
    buf3 db 512 dup(?)
    str_arg_0 db ' ',0
    str_arg_0_len dd 1
    str_arg_1 db 10,0
    str_arg_1_len dd 2
    str_any_2 db 'H','e','l','l','o',0
    str_any_3 db 'W','o','r','l','d','!',0

section '.text' code readable executable

func_GetStdHandle:
    sub rsp, 40
    invoke GetStdHandle, -11
    mov [hStdOut], rax
    invoke GetStdHandle, -10
    mov [hStdIn], rax
    add rsp, 40
.func_end_GetStdHandle:
    ret

func_ExitProcess:
    invoke ExitProcess, 0
.func_end_ExitProcess:
    ret

func_strlen:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    sub rsp, 40
    invoke lstrlenA, rcx
    add rsp, 40
.func_end_strlen:
    add rsp, 8
    ret

func___len:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    mov rax, [rcx - 8]
.func_end___len:
    add rsp, 8
    ret

func___val:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    mov rax, [rcx + 8]
.func_end___val:
    add rsp, 8
    ret

func___type:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    mov rax, [rcx]
.func_end___type:
    add rsp, 8
    ret

func_len:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    mov rcx, qword [rsp + 0]
    call func_strlen
    jmp .func_end_len
.func_end_len:
    add rsp, 8
    ret

func_atoi:
    push rbx
    push r12
    push r13
    push r14
    sub rsp, 8
    mov qword [rsp + 0], rcx
    ; let result: i32
    xor rbx, rbx
    ; let i: i32
    xor r12, r12
    ; let sign: i32
    mov r13, 1
    mov rdx, qword [rsp + 0]
    cmp byte [rdx + 0], 45
    jne .if_else_0
    neg r13
    ; increment i
    inc r12
.if_else_0:
.if_end_0:
.while_start_0:
    ; let c: i32
    mov rax, r12
    mov rdx, qword [rsp + 0]
    movzx r14, byte [rdx + rax]
    cmp r14, 0
    jne .if_else_1
    ; break loop
    jmp .while_end_0
.if_else_1:
.if_end_1:
    cmp r14, 48
    jge .if_else_2
    ; break loop
    jmp .while_end_0
.if_else_2:
.if_end_2:
    cmp r14, 57
    jle .if_else_3
    ; break loop
    jmp .while_end_0
.if_else_3:
.if_end_3:
    imul rbx, 10
    mov rax, r14
    sub rax, 48
    add rbx, rax
    ; increment i
    inc r12
    jmp .while_start_0
.while_end_0:
    jmp .func_end_atoi
.func_end_atoi:
    add rsp, 8
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

func_print_int:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 8
    mov qword [rsp + 0], rcx
    ; let n: i32
    mov rbx, qword [rsp + 0]
    ; let idx: i32
    xor r12, r12
    ; let divisor: i32
    mov r13, 1
    cmp rbx, 0
    jge .if_else_4
    neg rbx
    mov rax, 45
    mov byte [numBuffer + 0], al
    mov r12, 1
.if_else_4:
.if_end_4:
    ; let m: i32
    mov r14, rbx
.while_start_1:
    cmp r14, 9
    jle .while_end_1
    imul r13, 10
    mov r10, 10
    mov rax, r14
    xor rdx, rdx
    div r10
    mov r14, rax
    jmp .while_start_1
.while_end_1:
.while_start_2:
    ; let d: i32
    mov r10, r13
    mov rax, rbx
    xor rdx, rdx
    div r10
    mov r15, rax
    mov rax, r15
    add rax, 48
    mov byte [numBuffer + r12], al
    mov r10, r13
    mov rax, rbx
    xor rdx, rdx
    div r10
    mov rbx, rdx
    mov r10, 10
    mov rax, r13
    xor rdx, rdx
    div r10
    mov r13, rax
    add r12, 1
    cmp r13, 0
    jne .if_else_5
    ; break loop
    jmp .while_end_2
.if_else_5:
.if_end_5:
    jmp .while_start_2
.while_end_2:
    ; call function WriteConsole(hStdOut, numBuffer, idx, &bytesWritten)
    mov rcx, [hStdOut]
    lea rdx, [numBuffer]
    mov r8, r12
    lea r9, [bytesWritten]
    call func_WriteConsole
.func_end_print_int:
    add rsp, 8
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

func_WriteConsole:
    sub rsp, 32
    mov qword [rsp + 0], rcx
    mov qword [rsp + 8], rdx
    mov qword [rsp + 16], r8
    mov qword [rsp + 24], r9
    sub rsp, 40
    invoke WriteConsoleA, rcx, rdx, r8, r9, 0
    add rsp, 40
.func_end_WriteConsole:
    add rsp, 32
    ret

func_println:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 8
    mov qword [rsp + 0], rcx
    ; let i: i32
    xor rbx, rbx
    ; let count: i32 = __len ( args )
    mov rcx, qword [rsp + 0]
    call func___len
    mov r12, rax
.while_start_3:
    cmp rbx, r12
    jge .while_end_3
    ; let elem: i64
    mov rax, rbx
    mov rdx, qword [rsp + 0]
    imul rax, 8
    mov r13, [rdx + rax]
    ; let t: i32 = __type ( elem )
    mov rcx, r13
    call func___type
    mov r14, rax
    ; let v: i64 = __val ( elem )
    mov rcx, r13
    call func___val
    mov r15, rax
    cmp r14, 0
    jne .if_else_6
    ; call function print_int(v)
    mov rcx, r15
    call func_print_int
.if_else_6:
.if_end_6:
    cmp r14, 1
    jne .if_else_7
    ; call function printf(v)
    mov rcx, r15
    call func_printf
.if_else_7:
.if_end_7:
    ; call function printf(" ")
    lea rcx, [str_arg_0]
    call func_printf
    ; increment i
    inc rbx
    jmp .while_start_3
.while_end_3:
    ; call function printf("\n")
    lea rcx, [str_arg_1]
    call func_printf
.func_end_println:
    add rsp, 8
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

func_printf:
    push rbx
    sub rsp, 8
    mov qword [rsp + 0], rcx
    ; let len: u32 = strlen ( text )
    mov rcx, qword [rsp + 0]
    call func_strlen
    mov rbx, rax
    ; call function WriteConsole(hStdOut, text, len, &bytesWritten)
    mov rcx, [hStdOut]
    mov rdx, qword [rsp + 0]
    mov r8, rbx
    lea r9, [bytesWritten]
    call func_WriteConsole
.func_end_printf:
    add rsp, 8
    pop rbx
    ret

func_scanf:
    sub rsp, 8
    mov qword [rsp + 0], rcx
    ; call function ReadConsoleA(hStdIn, buf, 511, &charsRead, 0)
    mov rcx, [hStdIn]
    mov rdx, qword [rsp + 0]
    mov r8, 511
    lea r9, [charsRead]
    sub rsp, 8
    mov qword [rsp + 0], 0
    call [ReadConsoleA]
    add rsp, 8
    jmp .func_end_scanf
.func_end_scanf:
    add rsp, 8
    ret
start:
    sub rsp, 8
    and rsp, -16
    push rbx
    push r12
    sub rsp, 40
    ; call function GetStdHandle()
    call func_GetStdHandle
    ; let a: i32
    mov rbx, 10
    ; let b: i32
    mov r12, 20
    ; call function println(a, "Hello", b, "World!")
    sub rsp, 112
    mov r10, rsp
    mov qword [r10], 4
    lea r11, [r10 + 40]
    mov [r10 + 8], r11
    mov qword [r10 + 40], 0
    mov [r10 + 48], rbx
    lea r11, [r10 + 56]
    mov [r10 + 16], r11
    mov qword [r10 + 56], 1
    lea r11, [str_any_2]
    mov [r10 + 64], r11
    lea r11, [r10 + 72]
    mov [r10 + 24], r11
    mov qword [r10 + 72], 0
    mov [r10 + 80], r12
    lea r11, [r10 + 88]
    mov [r10 + 32], r11
    mov qword [r10 + 88], 1
    lea r11, [str_any_3]
    mov [r10 + 96], r11
    lea rcx, [r10 + 8]
    call func_println
    add rsp, 112
    ; call function ExitProcess()
    call func_ExitProcess
    add rsp, 40
    pop r12
    pop rbx

section '.idata' import data readable writeable
    library kernel32,'KERNEL32.DLL'
    import kernel32,\
        GetStdHandle,'GetStdHandle',\
        WriteConsoleA,'WriteConsoleA',\
        ReadConsoleA,'ReadConsoleA',\
        ReadConsoleW,'ReadConsoleW',\
        FlushConsoleInputBuffer,'FlushConsoleInputBuffer',\
        WideCharToMultiByte,'WideCharToMultiByte',\
        lstrlenA,'lstrlenA',\
        SetConsoleOutputCP,'SetConsoleOutputCP',\
        ExitProcess,'ExitProcess'
