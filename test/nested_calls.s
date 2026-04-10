
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

fun_a:
push rbp
mov rbp, rsp
mov rax, 2
mov rsp, rbp
pop rbp
ret

fun_b:
push rbp
mov rbp, rsp
sub rsp, 16
call fun_a
add rsp, 16
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

sub rsp, 16
call fun_b
add rsp, 16

mov rsp, rbp

pop rbp

ret
