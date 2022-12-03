lines    =: }: LF chopstring (1!:1 < 'input.txt')
intersection =: {{ ~. x #~ x e. y }}
chunks   =: >> _3 <\ lines
alphabet =: (a. {~ 97+i.26), (a. {~ 65+i.26)
echo +/1+alphabet i. ,/ }:"1 intersection/"2 chunks
exit ''
