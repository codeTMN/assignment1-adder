
section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 10
mov [rsp - 16], rax
mov rax, 20
mov [rsp - 24], rax
mov rax, [rsp - 24]
  ret
