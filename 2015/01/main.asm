bits 32
extern _print

section .data

print_final_floor:            db `Final floor: %d\n`, 0
print_first_entered_basement: db `First entered basement: %d\n`, 0

section .text

global _entryPoint

_entryPoint:
  enter 0,0
  push ebx
  push edi

  xor eax, eax         ; current floor
  xor ebx, ebx         ; current instruction
  xor ecx, ecx         ; instruction counter
  mov edx, [ ebp + 8 ] ; instruction pointer
  mov edi, -1          ; first entered basement (-1 = unset)

.instructionLoop:
  inc ecx         ; increment instruction counter
  mov bl, [ edx ] ; bl = instructions[edx]
  inc edx         ; increment instruction pointer

  cmp bl, '('       ; left parenthesis...
  je .moveUpFloor   ; moves up a floor
  cmp bl, ')'       ; right parenthesis...
  je .moveDownFloor ; moves down a floor
  jmp .end          ; unknown character skips to the end

.moveUpFloor:
  inc eax
  jmp .instructionLoop

.moveDownFloor:
  dec eax
  cmp eax, -1 ; check if we entered the basement
  je .onEnteredBasement
  jmp .instructionLoop

 .onEnteredBasement:
  cmp edi, -1          ; check if firstEnteredBasement has not been set yet
  jne .instructionLoop ; if it was already set, go to next instruction
  mov edi, ecx         ; set first entered basement to instruction counter
  jmp .instructionLoop

.end:
  push eax
  push dword print_final_floor
  call _print

  push edi
  push dword print_first_entered_basement
  call _print

  pop edi
  pop ebx
  leave
  ret
