@space
@data
@code
._entry:
  add                           ; This is a comment
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

