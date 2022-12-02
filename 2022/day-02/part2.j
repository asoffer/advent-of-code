text   =: (1!:1 < 'input.txt')
lines  =: 0 2&{&.|: _4 ]\ text

values =: (0&{&.|: I. (a.&=)"0 lines) - (|:>($&65;$&88) #lines)

score=: verb define
  (1+3|1-~+/y)+3*1{y
)

(+/ score"1 values) 1!:2 (2)
exit ''
