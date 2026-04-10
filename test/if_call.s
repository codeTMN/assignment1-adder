
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
mov rax, 3
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

sub rsp, 16
call fun_f
add rsp, 16
cmp rax, 1
je if_else_1
mov rax, 20
jmp if_end_0
if_else_1:
mov rax, 40
if_end_0:

mov rsp, rbp

pop rbp

ret
