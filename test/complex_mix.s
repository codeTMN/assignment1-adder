
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
  mov rax, 20
mov [rsp - 16], rax
mov rax, 3
mov [rsp - 24], rax
mov rax, [rsp - 16]
mov rbx, rax
and rbx, 1
shl rbx, 1
xor rbx, 3
mov rax, rbx
cmp rax, 1
je if_else_1
mov rax, [rsp - 24]
cmp rax, 1
je if_else_3
mov rax, [rsp - 16]
mov [rsp - 32], rax
mov rax, 10
mov rbx, rax
or rbx, [rsp - 32]
and rbx, 1
cmp rbx, 0
jne error_not_num
add rax, [rsp - 32]
jo error_overflow
jmp if_end_2
if_else_3:
mov rax, [rsp - 16]
if_end_2:
jmp if_end_0
if_else_1:
mov rax, 0
if_end_0:
  ret
