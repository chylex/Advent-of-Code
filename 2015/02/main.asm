bits 64
default rel

section .text

extern print
global entryPoint

entryPoint:
  push rbp
  mov rbp, rsp

  push rbx
  push r12
  push r13
  sub rsp, 32

  ; rax = reserved for math
  ; rdx = reserved for math

  ; rbx = value of current character of input string
  ; rcx = pointer to current character of input string
  ; r8  = return value for nextNumber
  ; r9  = first dimension
  ; r10 = second dimension
  ; r11 = third dimension
  ; r12 = total wrapping paper required
  ; r13 = total ribbon length required

  xor rdx, rdx
  xor r12, r12
  xor r13, r13

.nextLine:
  call .nextNumber
  mov r9, r8  ; save first dimension to r9
  call .nextNumber
  mov r10, r8 ; save second dimension to r10
  call .nextNumber
  mov r11, r8 ; save third dimension to r11

.sort1:        ; initiate bubble sort of the three dimensions (possible configurations 123; 132; 213; 231; 312; 321)
  cmp r9, r10  ; compare first pair of dimensions
  jb .sort2    ; if they are already sorted, skip ahead (123; 132; 231)
  xchg r9, r10 ; otherwise, swap the first pair (213->123; 312->132; 321->231)
.sort2:
  cmp r10, r11  ; compare last pair of dimensions
  jb .sort3     ; if they are already sorted, skip ahead (123)
  xchg r10, r11 ; otherwise, swap the last pair (132->123; 231->213)
.sort3:
  cmp r9, r10  ; compare first pair of dimensions again in case the smallest number started at the end and was moved to second position
  jb .sortEnd  ; if they are already sorted, skip ahead (123)
  xchg r9, r10 ; otherwise, swap the first pair (213->123)
.sortEnd:      ; at this point, the dimensions are sorted from smallest to largest in registers r9, r10, r11

  mov rax, 2
  mul r9
  mul r10
  add r12, rax ; add 2 * dim1 * dim2 to wrapping paper length

  mov rax, 2
  mul r9
  mul r11
  add r12, rax ; add 2 * dim1 * dim3 to wrapping paper length

  mov rax, 2
  mul r10
  mul r11
  add r12, rax ; add 2 * dim2 * dim3 to wrapping paper length

  mov rax, r9
  mul r10
  add r12, rax ; add square area of smallest side to wrapping paper length

  add r13, r9
  add r13, r9
  add r13, r10
  add r13, r10 ; add smallest perimeter to the ribbon length

  mov rax, r9
  mul r10
  mul r11
  add r13, rax ; add volume of the present to the ribbon length

  movzx rbx, byte [ rcx ] ; read current character
  cmp bl, 0               ; if we have not reached end of string yet,
  jne .nextLine           ; go read the next line

  lea rcx, [ print_wrapping_paper ]
  mov rdx, r12
  call print

  lea rcx, [ print_ribbon_length ]
  mov rdx, r13
  call print

  add rsp, 32
  pop r13
  pop r12
  pop rbx

  leave
  ret

.nextNumber:
  xor r8, r8 ; reset return value to zero

.nextDigit:
  movzx rbx, byte [ rcx ] ; read current character
  inc rcx                 ; move to next character

  cmp bl, '0'
  jb .nonDigitCharacter
  cmp bl, '9'
  ja .nonDigitCharacter

  mov rax, 10
  mul r8      ; shift digit accumulator by one decimal place
  mov r8, rax ; move multiplied result back to digit accumulator

  sub rbx, '0' ; convert ASCII to digit
  add r8, rbx  ; add digit to accumulator

  jmp .nextDigit

.nonDigitCharacter:
  ret ; return from nextNumber calls

section .data

print_wrapping_paper: db `Wrapping paper required: %d\n`, 0
print_ribbon_length:  db `Ribbon length required: %d\n`, 0
