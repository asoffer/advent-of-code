windowDiffs stride nums =
  zipWith (-) (drop stride nums) nums

countIncreases = length . (filter (> 0)) . (windowDiffs 3)

main = do
  contents <- readFile "input.txt"
  print $ countIncreases $ map read $ words contents
