  @data
  .array: 0 1 2 3 4 5 6 7

@code
._entry:
  call .main
  halt

.main:
  const 15                      ; How many times will we loop?
  call .goingup
  ret

.goingup:
  store 0                       ; Store this for later use
  const 1                       ; We'll start with 1
  'loop:
  print
  dup
  add
  dup
  dup
  store 1
  load 0
  const 1
  sub
  dup
  store 0
  jmpnz 'loop
  load 1
  call .backdown
  ret

.backdown:
  print
  const 2
  div
  dup
  'loop:
  print
  const 2
  div
  dup
  jmpnz 'loop
  ret
