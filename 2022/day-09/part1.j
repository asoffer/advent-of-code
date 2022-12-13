input      =: }: LF chopstring (1!:1 < 'input.txt')
directions =: 4 2 $1 0 0 _1 0 1 _1 0
counts     =: >".&.> 2&}.&.> input
steps      =: directions {~ 'DLRU' I. counts # >{.&.> input
headPos    =: (0 0), +/\ steps
moveTail   =: {{ (+./ 2< +/*:y) * (%|)y }} 
tailPos    =: |. {{ y+moveTail (x - y) }} /\. |. headPos
echo {.$~.tailPos
exit ''
