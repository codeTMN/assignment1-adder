
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
mov [rsp - 16], rax
mov rax, 3
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
add rax, [rsp - 16]
jo error_overflow
  ret
