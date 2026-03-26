
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
  mov rax, 10
mov rbx, rax
and rbx, 1
shl rbx, 1
xor rbx, 3
mov rax, rbx
  ret
