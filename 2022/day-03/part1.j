lines    =: }: LF chopstring (1!:1 < 'input.txt')
chunks   =: ($~ 2&,&-:&#) &.> lines
reduced  =: ~."1 &.> chunks
dups     =: ,/>I.@:e.~/ &.> reduced
repeats  =: dups } |: > {:&.> reduced
alphabet =: (a. {~ 97+i.26), (a. {~ 65+i.26)
echo +/1+alphabet i. repeats
exit ''
