
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
mov rax, 10
mov rbx, rax
xor rbx, [rsp - 16]
test rbx, 1
jne error_not_num
cmp rax, [rsp - 16]
mov rax, 1
mov rbx, 3
cmove rax, rbx
  ret
