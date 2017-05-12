; Setup the "registers"
const 0
store 0         ; .aggregate

const 10
store 1         ; .current_num

; Start the loop here
load 1          ; load .current_num
load 0          ; load .aggregate for addition
add
store 0         ; store result back to .aggregate
const 1
load 1          ; decrement current num by 1
sub
store 1         ; store new .current_num
load 1
const 0
jmp_rel_ne -47  ; jump back if the .current_num is 0
load 0
print           ; print the result and exit
halt
