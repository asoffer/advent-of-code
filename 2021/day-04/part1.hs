import Data.List

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
stepAndCheck num boards = let bs = step num boards in (bs, findIndex checkBoard bs)

firstWinningScore nums boards =
    let (bs, idx) = stepAndCheck (head nums) boards in 
        case idx of
            Nothing -> firstWinningScore (tail nums) bs
            Just i -> head nums * (score $ bs!!i)

parse s = let ls = lines s in (parseNumbers $ head ls, parseBoards $ tail ls)

main = do
  contents <- readFile "input.txt"
  print $ uncurry firstWinningScore $ parse contents
