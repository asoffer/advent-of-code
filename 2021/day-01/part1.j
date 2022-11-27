parseError =. _999
lines      =. (#~ ~:&parseError) >parseError ".&.> ;: 1!:1 boxopen 'input.txt'
result     =. +/ 0 > 2 -/\ lines

result 1!:2 (2)
exit ''
