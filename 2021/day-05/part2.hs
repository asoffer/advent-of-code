import Data.List
import Data.Maybe

cartesianProduct xs ys = [(x,y) | x <- xs, y <- ys]

x = fst
y = snd
x0 = x . fst
y0 = y . fst
x1 = x . snd
y1 = y . snd

rot90 = \(x, y) -> (y, -x)
diff = \(x0, y0) -> \(x1, y1) -> (x0 - x1, y0 - y1)
dotProd = \(x0, y0) -> \(x1, y1) -> x0 * x1 + y0 * y1

lineContains p l = let d1 = diff p (fst l) in
                   let d2 = diff p (snd l) in
                   0 == dotProd (rot90 d1) d2 &&
                   min (x0 l) (x1 l) <= (x p) &&
                   max (x0 l) (x1 l) >= (x p) &&
                   min (y0 l) (y1 l) <= (y p) &&
                   max (y0 l) (y1 l) >= (y p)

countLinesCrossing p = length . (filter (lineContains p))
multiCrosses ls p = 1 < (countLinesCrossing p) ls
countMultiCrosses ls ps = length $ (filter id) (map (multiCrosses ls) ps)

readInt = read :: String -> Int
parsePoint p = let (x, y) = splitAt (fromJust $ elemIndex ',' p) p in (readInt x, readInt $ tail y)
parseLine line = let w = words line in (parsePoint $ head w, parsePoint $ w!!2)

main = do
  contents <- readFile "input.txt"
  print $ countMultiCrosses (map parseLine $ lines contents) $ cartesianProduct [0..1000] [0..1000]
