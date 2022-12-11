input =: "."0>}: LF chopstring (1!:1 < 'input.txt')
view =: (1 i.~ ({. <: }.)) + ({. <: >./@:}.)
scores =: (view\."1 input) * (view\."1 &.|: input) * (view\."1 &.|."1 input) * (view\."1 &.(|:@:|.) input)
echo >./,scores
exit ''
