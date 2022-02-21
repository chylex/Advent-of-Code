bits 64
default rel

section .text

extern print
extern malloc
extern free
global entryPoint

entryPoint:
  push rbp
  mov rbp, rsp

  push rbx
  push r12
  push r13
  push r14
  push r15
  sub rsp, 32

  call findCoordinateRanges
  call allocateGridArray
  call zeroGridArray
  call deliverPresents

  mov rcx, r13
  call free

  lea rcx, [ print_houses_with_at_least_one_present ]
  mov rdx, r12
  call print

  add rsp, 32
  pop r15
  pop r14
  pop r13
  pop r12
  pop rbx

  leave
  ret

findCoordinateRanges:

  ; TMP | rbx = value of current character of input string
  ;  IN | rcx = pointer to current character of input string
  ; TMP | r8  = x coordinate
  ; TMP | r9  = y coordinate
  ; OUT | r10 = minimum x
  ; OUT | r11 = minimum y
  ; OUT | r12 = maximum x
  ; OUT | r13 = maximum y
  ; TMP | r14 = next x offset
  ; TMP | r15 = next y offset

  push rcx ; remember the beginning of the input string

  xor r8, r8
  xor r9, r9
  xor r10, r10
  xor r11, r11
  xor r12, r12
  xor r13, r13
  xor r14, r14
  xor r15, r15

.nextChar:
  movzx rbx, byte [ rcx ] ; read current character
  inc rcx                 ; move to next character

  cmp bl, '^'
  je .goUp

  cmp bl, 'v'
  je .goDown

  cmp bl, '<'
  je .goLeft

  cmp bl, '>'
  je .goRight

  pop rcx ; for any other character, reset rcx to the beginning of the input string...
  ret     ; and return from the procedure

.goUp:
  xor r14, r14
  mov r15, -1
  jmp .updateCoordinates

.goDown:
  xor r14, r14
  mov r15, 1
  jmp .updateCoordinates

.goLeft:
  mov r14, -1
  xor r15, r15
  jmp .updateCoordinates

.goRight:
  mov r14, 1
  xor r15, r15
  ; fall through

.updateCoordinates:
  add r8, r14 ; add x offset to the x coordinate
  add r9, r15 ; add y offset to the y coordinate

  cmp   r10, r8 ; compare the minimum x with the current x coordinate
  cmovg r10, r8 ; if the minimum is greater than x, set it to x

  cmp   r11, r9 ; compare the minimum y with the current y coordinate
  cmovg r11, r9 ; if the minimum is greater than y, set it to y

  cmp   r12, r8 ; compare the maximum x with the current x coordinate
  cmovl r12, r8 ; if the maximum is lesser than x, set it to x

  cmp   r13, r9 ; compare the maximum y with the current y coordinate
  cmovl r13, r9 ; if the maximum is lesser than y, set it to y

  jmp .nextChar

allocateGridArray:

  ; TMP | rax = reserved for math
  ;  IN | rcx = pointer to current character of input string
  ; OUT | r8  = width of the grid
  ; OUT | r9  = height of the grid
  ;  IN | r10 = minimum x
  ; OUT | r10 = x coordinate where Santa starts
  ;  IN | r11 = minimum y
  ; OUT | r11 = y coordinate where Santa starts
  ;  IN | r12 = maximum x
  ;  IN | r13 = maximum y
  ; OUT | r13 = pointer to beginning of grid array (2 bytes per cell)
  ; OUT | r14 = total size of the grid array in bytes

  mov r8, r12 ; r8 = maximum x
  sub r8, r10 ; r8 -= minimum x
  inc r8      ; r8 += 1

  mov r9, r13 ; r9 = maximum y
  sub r9, r11 ; r9 -= minimum y
  inc r9      ; r9 += 1

  mov rax, r8
  mul r9       ; rax = total width times total height, i.e. total grid area
  add rax, rax ; rax = rax * 2 to store up to 2 bytes per grid cell

  neg r10 ; Santa's new center on the x axis is the negated value of minimum x
  neg r11 ; Santa's new center on the y axis is the negated value of minimum y

  push rcx
  push r8
  push r9
  push r10
  push r11
  push rax ; grid array size will be popped into r14 later

  sub rsp, 8
  mov rcx, rax
  call malloc  ; allocate grid array and store a pointer to the beginning in rax
  mov r13, rax ; move grid pointer to r13
  add rsp, 8

  pop r14
  pop r11
  pop r10
  pop r9
  pop r8
  pop rcx
  ret

zeroGridArray:

  ; TMP | rax = pointer to current position in the grid array
  ;  IN | r13 = pointer to beginning of grid array (2 bytes per cell)
  ;  IN | r14 = total size of the grid array in bytes
  ; TMP | r15 = remaining bytes to clear

  mov rax, r13
  mov r15, r14

.loop:
  cmp r15, 0
  jle .done

  mov byte [ rax ], 0
  inc rax
  dec r15
  jmp .loop

.done:
  ret

deliverPresents:

  ; TMP | rax = offset in the grid array
  ; TMP | rbx = value of current character of input string
  ;  IN | rcx = pointer to current character of input string
  ;  IN | r8  = width of the grid
  ;  IN | r9  = height of the grid
  ;  IN | r10 = x coordinate, initially set to where Santa starts
  ;  IN | r11 = y coordinate, initially set to where Santa starts
  ; OUT | r12 = amount of houses that received at least one present
  ;  IN | r13 = pointer to beginning of grid array (2 bytes per cell)
  ; TMP | r14 = next x offset
  ; TMP | r15 = next y offset

  xor r12, r12

  jmp .deliverPresent ; deliver first present to house at Santa's current location

.nextChar:
  movzx rbx, byte [ rcx ] ; read current character
  inc rcx                 ; move to next character

  cmp bl, '^'
  je .goUp

  cmp bl, 'v'
  je .goDown

  cmp bl, '<'
  je .goLeft

  cmp bl, '>'
  je .goRight

  ret ; for any other character, return from the procedure

.goUp:
  xor r14, r14
  mov r15, -1
  jmp .moveAndDeliverPresent

.goDown:
  xor r14, r14
  mov r15, 1
  jmp .moveAndDeliverPresent

.goLeft:
  mov r14, -1
  xor r15, r15
  jmp .moveAndDeliverPresent

.goRight:
  mov r14, 1
  xor r15, r15
  ; fall through

.moveAndDeliverPresent:
  add r10, r14 ; add x offset to the x coordinate
  add r11, r15 ; add y offset to the y coordinate

.deliverPresent:
  mov rax, r11 ; rax = y
  mul r8       ;       y * grid width
  add rax, r10 ;       y * grid width + x
  add rax, rax ; double the offset because every cell is 2 bytes

  inc word [ r13 + rax ]
  cmp word [ r13 + rax ], 1
  je .firstPresentForHouse
  jmp .nextChar

.firstPresentForHouse:
  inc r12
  jmp .nextChar

section .data

print_houses_with_at_least_one_present: db `Total houses with at least one present: %d\n`, 0
