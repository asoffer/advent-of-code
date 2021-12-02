ascii       =: a. & i.
digitValue  =: _48 + ascii
powerOfTen  =: 10&^ @ |. @ i. @ $
parseNumber =: +/ @ (powerOfTen * digitValue)
strsplit    =: #@[ }.each [ (E. <;.1 ]) ,
input       =: parseNumber @ > LF strsplit 1!:1@boxopen 'input.txt'
windows     =: 3 +/\ input
diffs       =: 2 -/\ windows
result      =: +/ 0 > diffs

result 1!:2 (2)
exit ''
