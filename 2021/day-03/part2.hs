import Data.List

unBitSeqImpl n b = 2 * n + (if b then 1 else 0)
unBitSeq :: [Bool] -> Int
unBitSeq = foldl unBitSeqImpl 0

moreCommonWithDefault d l = let twiceOnes = (2 * (length $ filter id l))
                       in let total = length l
                       in if twiceOnes < total then False else if twiceOnes > total then True else d
lessCommonWithDefault d l = let twiceOnes = (2 * (length $ filter id l))
                       in let total = length l
                       in if twiceOnes < total then True else if twiceOnes > total then False else d

pick f xs = let b = f $ map head xs in (map tail $ filter ((b ==) . head) xs, b)

untilOneLeft f x = case f x of
    ([], _) -> ([], head x)
    ([a], b) -> ([b], a)
    (as, b) -> let (bs, a) = untilOneLeft f as in (b:bs, a)

pickForOxygen = moreCommonWithDefault True
pickForCO2 = lessCommonWithDefault False
oxygen = unBitSeq . uncurry (++) . (untilOneLeft $ pick pickForOxygen)
co2Scrub = unBitSeq . uncurry (++) . (untilOneLeft $ pick pickForCO2)

parse = map (map (== '1')) . lines
compute x = oxygen x * co2Scrub x 

main = do
  contents <- readFile "input.txt"
  print $ compute $ parse contents
