
section .text
extern snek_error
global our_code_starts_here

error_not_num:
  mov rdi, 1
  call snek_error

error_overflow:
  mov rdi, 2
  call snek_error

our_code_starts_here:
  mov rax, 0
cmp rax, 1
je if_else_1
mov rax, 20
jmp if_end_0
if_else_1:
mov rax, 40
if_end_0:
  ret
