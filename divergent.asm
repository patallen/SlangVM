; Setup the "registers"
  const 0
.aggregate
  store 0                   ; .aggregate

  const 10
.current_num
  store 1                   ; .current_num

.main_loop
  load 1                    ; load .current_num
  load 0                    ; load .aggregate for addition
  add
  store 0                   ; store result back to .aggregate
  const 1
  load 1                    ; decrement current num by 1
  sub
  store 1                   ; store new .current_num
  load 1
  const 0
  jmp_rel_ne .main_loop     ; jump back if the .current_num is 0
  load 0
  print                     ; print the result and exit
  halt
