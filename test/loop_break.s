
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
  loop_start_0:
mov rax, 84
jmp loop_end_1
jmp loop_start_0
loop_end_1:
  ret
