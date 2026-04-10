
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

fun_f:
push rbp
mov rbp, rsp
mov rax, [rbp + 48]
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
mov rax, 8
mov [rsp - 40], rax
mov rax, 10
mov [rsp - 48], rax
sub rsp, 56
sub rsp, 8
mov rax, [rsp + 16]
push rax
mov rax, [rsp + 32]
push rax
mov rax, [rsp + 48]
push rax
mov rax, [rsp + 64]
push rax
mov rax, [rsp + 80]
push rax
call fun_f
add rsp, 104

mov rsp, rbp

pop rbp

ret
