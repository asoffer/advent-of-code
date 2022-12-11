input =: "."0>}: LF chopstring (1!:1 < 'input.txt')
mask =: {{ }:_1|. (~: 1&|.) _1,~ >./\ y }}
echo +/ ,(mask"1 input) +. (mask"1 &.|: input) +. (mask"1 &.|."1 input) +. (mask"1 &.(|:@:|.) input)
