
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
mov [rsp - 16], rax
loop_start_0:
mov rax, [rsp - 16]
mov [rsp - 24], rax
mov rax, 10
mov rbx, rax
xor rbx, [rsp - 24]
test rbx, 1
jne error_not_num
cmp rax, [rsp - 24]
mov rax, 1
mov rbx, 3
cmove rax, rbx
cmp rax, 1
je if_else_3
mov rax, [rsp - 16]
jmp loop_end_1
jmp if_end_2
if_else_3:
mov rax, [rsp - 16]
mov [rsp - 24], rax
mov rax, 2
mov rbx, rax
or rbx, [rsp - 24]
and rbx, 1
cmp rbx, 0
jne error_not_num
add rax, [rsp - 24]
jo error_overflow
mov [rsp - 16], rax
if_end_2:
jmp loop_start_0
loop_end_1:
  ret
