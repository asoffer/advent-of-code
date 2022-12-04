lines   =: }: LF chopstring (1!:1 < 'input.txt')
numbers =: > ".&.> > ','&chopstring &.> '-,'&charsub &.> lines
echo +/ (((1&{ >: 2&{) *. (0&{ <: 3&{)) +. ((3&{ >: 0&{) *. (2&{ <: 1&{)))"1 numbers
exit ''
