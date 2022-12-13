input   =: }: LF chopstring (1!:1 < 'input.txt')
NB. Expand every value to how much needs to be added at the end of it's cycle.
updates =: > 0&". &.> ; ' '&chopstring &.> input
values  =: 1,1+ +/\ updates
pos     =: ($values) $ i.40
echo 6 40 $ (2>|values - pos) { ' #'

