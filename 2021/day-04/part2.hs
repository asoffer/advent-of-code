import Data.List
import Data.Maybe

reshape n l = if length l <= n then [l] else (take n l) : (reshape n $ drop n l)

wordsBy p l = case findIndex p l of
    Nothing -> [l]
    Just n -> let (h, t) = splitAt n l in (h:(wordsBy p $ tail t))
readInt = read :: String -> Int
parseBoards = (map (map (map readInt . words) . tail)) . reshape 6
parseNumbers = map readInt . (wordsBy (== ','))

setNumber n x = if n == x then -1 else x
markNumber = map . map . setNumber

checkRow = (== -5) . foldl1 (+)
checkRows = foldl1 (||) . map checkRow
checkBoard b = checkRows b || (checkRows $ transpose b)

score :: [[Int]] -> Int
score = (foldl1 (+)) . (map (foldl1 (+))) . ((map . map . max) 0)

step :: Int -> [[[Int]]] -> [[[Int]]]
step = map . markNumber
stepAndFilter num = (filter (not . checkBoard)) . (step num)
lastWinningScore nums boards = 
    let bs = stepAndFilter (head nums) boards in if length bs == 0
        then head nums * (score $ markNumber (head nums) $ fromJust $ find (not . checkBoard) boards)
        else lastWinningScore (tail nums) bs


parse s = let ls = lines s in (parseNumbers $ head ls, parseBoards $ tail ls)

main = do
  contents <- readFile "input.txt"
  print $ uncurry lastWinningScore $ parse contents
