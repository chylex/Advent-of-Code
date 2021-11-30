bits 32

section .data
section .text

global _walkFloors

_walkFloors:
  enter 0,0
  push ebx

  mov eax, [ ebp +  8 ] ; eax = currentFloor
  mov edx, [ ebp + 12 ] ; edx = char* instructions
  mov ecx, [ ebp + 16 ] ; ecx = size_t instructionCount

  jcxz .end ; skip to end if there are no instructions

  inc ecx ; increment instruction count by 1
          ; because loop instruction uses prefix decrement

.instructionLoop:
  mov bl, [ edx ] ; bl = instructions[edx]
  inc edx         ; edx++

  cmp bl, '('     ; left parenthesis...
  je .moveUpFloor ; moves up a floor

  cmp bl, ')'       ; right parenthesis...
  je .moveDownFloor ; moves down a floor

  jmp .end ; unknown character skips to the end

.moveUpFloor:
  inc eax
  loop .instructionLoop

.moveDownFloor:
  dec eax
  loop .instructionLoop

.end:
  pop ebx
  leave
  ret
