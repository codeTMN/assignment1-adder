
section .text
extern snek_error
extern snek_print
global our_code_starts_here

error_not_num:
  mov rdi, 1
  mov rbx, rsp
  and rsp, -16
  call snek_error
  mov rsp, rbx

error_overflow:
  mov rdi, 2
  mov rbx, rsp
  and rsp, -16
  call snek_error
  mov rsp, rbx

fun_is_even:
push rbp
mov rbp, rsp
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, 0
mov rbx, rax
xor rbx, [rsp - 16]
test rbx, 1
jne error_not_num
cmp rax, [rsp - 16]
mov rax, 1
mov rbx, 3
cmove rax, rbx
cmp rax, 1
je if_else_1
mov rax, 3
jmp if_end_0
if_else_1:
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, 2
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
mov rbx, rax
mov rax, [rsp - 16]
sub rax, rbx
jo error_overflow
mov [rsp - 16], rax
sub rsp, 24
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_is_odd
add rsp, 40
if_end_0:
mov rsp, rbp
pop rbp
ret

fun_is_odd:
push rbp
mov rbp, rsp
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, 0
mov rbx, rax
xor rbx, [rsp - 16]
test rbx, 1
jne error_not_num
cmp rax, [rsp - 16]
mov rax, 1
mov rbx, 3
cmove rax, rbx
cmp rax, 1
je if_else_3
mov rax, 1
jmp if_end_2
if_else_3:
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, 2
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
mov rbx, rax
mov rax, [rsp - 16]
sub rax, rbx
jo error_overflow
mov [rsp - 16], rax
sub rsp, 24
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_is_even
add rsp, 40
if_end_2:
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

mov rax, 8
mov [rsp - 16], rax
sub rsp, 24
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_is_even
add rsp, 40

mov rsp, rbp

pop rbp

ret
