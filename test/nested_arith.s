
section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 2
mov [rsp - 16], rax
mov rax, 3
imul rax, [rsp - 16]
mov [rsp - 16], rax
mov rax, 10
mov [rsp - 24], rax
mov rax, 4
mov rbx, rax
mov rax, [rsp - 24]
sub rax, rbx
add rax, [rsp - 16]
  ret
