parseError =: _999
text =: (1!:1 < 'input.txt')
iotaRange =: 0&{ }. [: i. 1&{
incrementFirst =: (1+0&{), 1&{
lines =: text&({~)@iotaRange@incrementFirst &.> 2<\_1,I. LF=text
values =: parseError ".&.> '0'&,&.> lines
chunks =: ;&.> values&({~)@iotaRange@incrementFirst&.> 2<\_1,I.>0=&.>values
someOfThreeLargest =: +/ (i.3) { ({~\:) > +/ &.> chunks
someOfThreeLargest 1!:2 (2)
