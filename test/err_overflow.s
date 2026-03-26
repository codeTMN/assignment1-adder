
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
  mov rax, 2000000000
mov [rsp - 16], rax
mov rax, 2000000000
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
sar rax, 1
imul rax, [rsp - 16]
jo error_overflow
  ret
