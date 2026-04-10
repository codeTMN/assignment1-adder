
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

fun_fact:
push rbp
mov rbp, rsp
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, 2
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
mov rax, 2
jmp if_end_0
if_else_1:
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, [rbp + 16]
mov [rsp - 24], rax
mov rax, 2
mov rbx, rax
or rbx, [rsp - 24]
and rbx, 1
cmp rbx, 0
jne error_not_num
mov rbx, rax
mov rax, [rsp - 24]
sub rax, rbx
jo error_overflow
mov [rsp - 24], rax
sub rsp, 32
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_fact
add rsp, 48
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
sar rax, 1
imul rax, [rsp - 16]
jo error_overflow
if_end_0:
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

mov rax, 10
mov [rsp - 16], rax
sub rsp, 24
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_fact
add rsp, 40

mov rsp, rbp

pop rbp

ret
