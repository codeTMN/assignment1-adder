
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
loop_start_2:
mov rax, 2
jmp loop_end_3
jmp loop_start_2
loop_end_3:
mov rax, 4
jmp loop_end_1
jmp loop_start_0
loop_end_1:
  ret
