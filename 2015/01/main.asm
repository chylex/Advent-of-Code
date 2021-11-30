bits 32

section .data
section .text

global _walkFloors

_walkFloors:
  enter 0,0
  push ebx
  push edi

  ; [ ebp +  8 ] = int currentFloor
  ; [ ebp + 12 ] = int* totalInstructionCounter
  ; [ ebp + 16 ] = int* firstEnteredBasement
  ; [ ebp + 20 ] = char* instructions
  ; [ ebp + 24 ] = size_t instructionCount

  mov eax, [ ebp +  8 ] ; eax = currentFloor
  mov edx, [ ebp + 20 ] ; edx = instructions
  xor ecx, ecx          ; ecx = 0

.instructionLoop:
  cmp ecx, [ ebp + 24 ] ; check if we went past instructionCount
  jge .end              ; if so, we are done here
  inc ecx               ; otherwise, increment instruction counter and continue

  mov bl, [ edx ] ; bl = instructions[edx]
  inc edx         ; edx++

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
  mov ebx, [ ebp + 16 ] ; ebx = &firstEnteredBasement
  cmp dword [ ebx ], -1 ; check if firstEnteredBasement has not been set yet
  jne .instructionLoop  ; if it was already set, go to next instruction

  mov edi, [ ebp + 12 ]  ; edi = &totalInstructionCounter
  mov edi, [ edi ]       ; edi = totalInstructionCounter
  add edi, ecx           ; add current instruction counter to the total
  mov dword [ ebx ], edi ; set firstEnteredBasement to the total
  jmp .instructionLoop

.end:
  mov edi, [ ebp + 12 ]  ; edi = &totalInstructionCounter
  mov ebx, [ edi ]       ; ebx = totalInstructionCounter
  add ebx, ecx           ; add current instruction counter to the total
  mov dword [ edi ], ebx ; set totalInstructionCounter to the new total

  pop edi
  pop ebx
  leave
  ret
