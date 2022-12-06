applyintree =: 2 : 0
  if. #n do. ((u applyintree (}.n)) L:_1 ({.n){y) ({.n)} y else. u y end.
  :
  if. #n do. (x u applyintree (}.n) L:_ _1 ({.n){y) ({.n)} y else. x u y end.
)

lines        =: }: LF chopstring (1!:1 < 'input.txt')
splitPoint   =: {. I. (<'') = lines
diagram      =: > (splitPoint - 1) {. lines
columns      =: (1 + 1 { $ diagram)%4
diagram      =: (#~~:&' ')&.><"1|."1|:((1 + 4 * i. columns)&{)"1 diagram
instructions =: 1 -~ > ".&.> 1 3 5 {"1 chopstring"1 > (splitPoint + 1) }. lines

choose       =: {{ {: > x { y }}
applyOnce    =: {{
  choice =. (1{x) choose y
  ,&choice applyintree (2{x) }: applyintree (1{x) y
}}
apply =: {{ x applyOnce^:(1 + {.x) y }}

exec =: dyad define
  d =. y
  for_ijk. x do.
    d =. ijk apply d
  end.
  d
)

echo > {: &.> instructions exec diagram
exit ''
