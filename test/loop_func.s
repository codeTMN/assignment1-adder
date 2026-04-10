
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
loop_start_0:
mov rax, [rbp + 16]
jmp loop_end_1
jmp loop_start_0
loop_end_1:
mov rsp, rbp
pop rbp
ret

our_code_starts_here:

push rbp

mov rbp, rsp

mov rax, 84
mov [rsp - 16], rax
sub rsp, 24
sub rsp, 8
mov rax, [rsp + 16]
push rax
call fun_f
add rsp, 40

mov rsp, rbp

pop rbp

ret
