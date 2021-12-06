import Data.List
import Data.Maybe

cartesianProduct xs ys = [(x,y) | x <- xs, y <- ys]

x = fst
y = snd
x0 = x . fst
y0 = y . fst
x1 = x . snd
y1 = y . snd
inRange val a b = (val <= max a b) && (val >= min a b)

lineContains p l = ((x p == x0 l) && (x p == x1 l) && inRange (y p) (y0 l) (y1 l)) ||
                   ((y p == y0 l) && (y p == y1 l) && inRange (x p) (x0 l) (x1 l))

countLinesCrossing p = length . (filter (lineContains p))
multiCrosses ls p = 1 < (countLinesCrossing p) ls
countMultiCrosses ls ps = length $ (filter id) (map (multiCrosses ls) ps)

readInt = read :: String -> Int
parsePoint p = let (x, y) = splitAt (fromJust $ elemIndex ',' p) p in (readInt x, readInt $ tail y)
parseLine line = let w = words line in (parsePoint $ head w, parsePoint $ w!!2)

axisAligned :: ((Int, Int), (Int, Int)) -> Bool
axisAligned l = (x0 l == x1 l) || (y0 l == y1 l)
validLines = filter axisAligned

main = do
  contents <- readFile "input.txt"
  print $ countMultiCrosses (validLines $ map parseLine $ lines contents) $ cartesianProduct [0..1000] [0..1000]
