import Data.List

count value = length . filter (== value) 
mostly value l = 2 * (count value l) > length l

unBitSeqImpl n b = 2 * n + (if b then 1 else 0)
unBitSeq :: [Bool] -> Int
unBitSeq = foldl unBitSeqImpl 0



parse = map (map (== '1')) . lines
epsilon = unBitSeq . map (mostly True) . transpose
gamma   = unBitSeq . map (mostly False) . transpose
compute x = gamma x * epsilon x 

main = do
  contents <- readFile "input.txt"
  print $ compute $ parse contents
