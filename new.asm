
@data
.numbers: 10 20 30 40
.empty: 0 0 0 0 0 0 0 0 0 0

@code
._entry:
  call .main
  print
  halt

  .main:
  const .numbers
  ret

