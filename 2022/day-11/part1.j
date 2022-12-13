input =: LF&chopstring &.> LF2 splitnostring (1!:1 < 'input.txt')

lastNumber =: {{ ". (- ' '&(i.~) &.|. y) {. y }}
throwSpec  =: {{ >lastNumber&.> 3 4 5 { y }}
throw      =: {{ (2 - 0 = (0 { y) | x) { y }}

applyMonkey =: {{
  monkeyId =. 0{y
  value =. 1{y
  entry =. >monkeyId{input
  op =: ' ' chopstring (}.~ 2+i.&'=') >2{ entry
  if. 'old' -: >0{op do.
    l =. value
  else.
    l =. ". >0{op
  end.

  if. 'old' -: >2{op do.
    r =. value
  else.
    r =. ". >2{op
  end.

  if. '*' = >1{op do.
    result=:<. 3 %~ l*r
  elseif. '+' = >1{op do.
    result=:<. 3 %~ l+r
  end.

  (result throw throwSpec entry), result
}}

items =: ".&.> (}.~ 2+ i.&':') &.> >1&{ &.> input

erase =: {{
  joined =. ((y = i. $x) { ]`'',}.`'') ,. x
  {{ < (0{y) `:0 >1{y }}"1 joined
}}

NB. Takes x=(table, counts) of held values, y=monkeyId.
NB. Returns a new table and new counts
pass =: {{ 
  oldMonkeyId =. y
  oldValue    =. (0;oldMonkeyId;0){:: 0{x
  counts      =. 1{x
  result      =. applyMonkey y,oldValue
  newMonkeyId =. 0{result
  newValue    =. 1{result
  table       =. (>0{x) erase oldMonkeyId
  additions   =. (newMonkeyId = i. $table) { a:,<newValue
  table       =. (table ,&.>"1 additions)
  (<table), (< (oldMonkeyId = i.$table) + >counts)
}}

passAll =: {{ > pass~&.>/ (($(0;y){::x)$<y),<x }}
round =: {{ > passAll~&.>/ (|. <"0 i. $>0{y),<y }}
sortedCounts =: (\:~) >{:{: round^:(i.21) items;<($items)$0
echo */ 2{. sortedCounts
exit ''
