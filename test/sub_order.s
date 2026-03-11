
section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 10
mov [rsp - 16], rax
mov rax, 2
mov rbx, rax
mov rax, [rsp - 16]
sub rax, rbx
mov [rsp - 16], rax
mov rax, 3
mov rbx, rax
mov rax, [rsp - 16]
sub rax, rbx
  ret
