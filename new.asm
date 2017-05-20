
@data
.numbers: 10 20 30 40
.empty: 0 0 0 0 0 0 0 0 0 0

@code
._entry:
  add                           ; This is a comment
  'loop:
  noop
  jmp 'loop
  store 0                       ; This is also a comment
  call .glabel
  halt

.glabel:
  store 0
  const 1
  const 10
  load 0
  add
  sub
  print
  ret

