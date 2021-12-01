adjacentDiffs nums = zipWith (-) (tail nums) nums

countIncreases = length . (filter (> 0)) . adjacentDiffs

main = do
  contents <- readFile "input.txt"
  print $ countIncreases $ map read $ words contents
