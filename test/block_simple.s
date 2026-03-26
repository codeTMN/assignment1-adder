
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
cmp rbx, 0
jne error_not_num
add rax, 2
jo error_overflow
mov rax, 20
mov rbx, rax
and rbx, 1
cmp rbx, 0
jne error_not_num
sub rax, 2
jo error_overflow
  ret
