bits 64
default rel

section .text

extern part1
extern part2
global entryPoint

entryPoint:
  push rbp
  mov rbp, rsp

  push rcx
  call part1

  pop rcx
  call part2

  leave
  ret
