const 5                         ; Load the argument to the stack
call .factorial                 ; Call the factorial subroutine
print                           ; Print the value on the top of the stack
halt                            ; Terminate the program


.factorial
  dup
  store 200

.fill_loop                      ; Fill loop through and store the the numbers to be multiplied (5, 4, 3, 2)
  dup
  const 1
  sub
  dup
  const 2
  jmp_rel_ne .fill_loop
  load 200

.mult_loop                      ; Loop x & multiply n - 1 times (actually n - 2... skipping 1)
  store 200
  mul
  load 200
  const 1
  sub
  dup
  const 2
  jmp_rel_ne .mult_loop
  store 99
  ret
