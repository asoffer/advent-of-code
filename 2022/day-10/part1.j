input   =: }: LF chopstring (1!:1 < 'input.txt')
NB. Expand every value to how much needs to be added at the end of it's cycle.
updates =: > 0&". &.> ; ' '&chopstring &.> input
NB. Add a starting value of 1 and account for the fact that the program counter
NB. starts at 1 but J indexes from zero.
values  =: 1,1,1+ +/\ updates
indices =: 20+40*i.6
echo +/ indices ([*{) values
exit ''
