bits 64
default rel

section .text

extern print
global entryPoint

entryPoint:
  push rbp
  mov rbp, rsp

  push rbx
  push rdi
  sub rsp, 32

  mov rdx, rcx ; instruction pointer
  xor rax, rax ; current floor
  xor rbx, rbx ; current instruction
  xor rcx, rcx ; instruction counter
  mov rdi, -1  ; first entered basement (-1 = unset)

.instructionLoop:
  inc rcx                 ; increment instruction counter
  movzx rbx, byte [ rdx ] ; bl = instructions[rdx]
  inc rdx                 ; increment instruction pointer

  cmp bl, '('       ; left parenthesis...
  je .moveUpFloor   ; moves up a floor
  cmp bl, ')'       ; right parenthesis...
  je .moveDownFloor ; moves down a floor
  jmp .end          ; unknown character skips to the end

.moveUpFloor:
  inc rax
  jmp .instructionLoop

.moveDownFloor:
  dec rax
  cmp rax, -1 ; check if we entered the basement
  je .onEnteredBasement
  jmp .instructionLoop

 .onEnteredBasement:
  cmp rdi, -1          ; check if firstEnteredBasement has not been set yet
  jne .instructionLoop ; if it was already set, go to next instruction
  mov rdi, rcx         ; set first entered basement to instruction counter
  jmp .instructionLoop

.end:
  lea rcx, [ print_final_floor ]
  mov rdx, rax
  call print

  lea rcx, [ print_first_entered_basement ]
  mov rdx, rdi
  call print

  add rsp, 32
  pop rdi
  pop rbx

  leave
  ret

section .data

print_final_floor:            db `Final floor: %d\n`, 0
print_first_entered_basement: db `First entered basement: %d\n`, 0
