
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

our_code_starts_here:

push rbp

mov rbp, rsp

mov rax, 3
sub rsp, 16
mov rdi, rax
mov rbx, rsp
and rsp, -16
call snek_print
mov rsp, rbx
add rsp, 16

mov rsp, rbp

pop rbp

ret
