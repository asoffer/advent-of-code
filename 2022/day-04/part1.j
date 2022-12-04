lines   =: }: LF chopstring (1!:1 < 'input.txt')
numbers =: > ".&.> > ','&chopstring &.> '-,'&charsub &.> lines
echo +/ (((0&{ <: 2&{)*.(1&{ >: 3&{)) +. ((0&{ >: 2&{)*.(1&{ <: 3&{)))"1 numbers
exit ''
