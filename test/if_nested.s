
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
  mov rax, 6
mov [rsp - 16], rax
mov rax, 10
mov rbx, rax
or rbx, [rsp - 16]
and rbx, 1
cmp rbx, 0
jne error_not_num
cmp [rsp - 16], rax
mov rax, 1
mov rbx, 3
cmovl rax, rbx
cmp rax, 1
je if_else_1
mov rax, 2
mov [rsp - 16], rax
mov rax, 2
mov rbx, rax
xor rbx, [rsp - 16]
test rbx, 1
jne error_not_num
cmp rax, [rsp - 16]
mov rax, 1
mov rbx, 3
cmove rax, rbx
cmp rax, 1
je if_else_3
mov rax, 200
jmp if_end_2
if_else_3:
mov rax, 400
if_end_2:
jmp if_end_0
if_else_1:
mov rax, 600
if_end_0:
  ret
