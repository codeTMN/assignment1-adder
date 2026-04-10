
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

fun_add3:
push rbp
mov rbp, rsp
mov rax, [rbp + 16]
mov [rsp - 16], rax
mov rax, [rbp + 24]
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
add rax, [rsp - 16]
jo error_overflow
mov [rsp - 16], rax
mov rax, [rbp + 32]
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
add rax, [rsp - 16]
jo error_overflow
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

mov rax, 2
mov [rsp - 16], rax
mov rax, 4
mov [rsp - 24], rax
mov rax, 6
mov [rsp - 32], rax
sub rsp, 40
sub rsp, 8
mov rax, [rsp + 16]
push rax
mov rax, [rsp + 32]
push rax
mov rax, [rsp + 48]
push rax
call fun_add3
add rsp, 72

mov rsp, rbp

pop rbp

ret
