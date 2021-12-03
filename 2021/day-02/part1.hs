import Data.List
import Data.Maybe

data Command =
  Forward Int |
  Down Int |
  Up Int

data State = State Int Int

try :: [a -> Maybe b] -> a -> Maybe b
try (f:fs) a = if isJust $ f a then f a else try fs a
try _ a = Nothing
  
parseCommand = try [
  fmap Forward . fmap read . (stripPrefix "forward "),
  fmap Down . fmap read . (stripPrefix "down "),
  fmap Up . fmap read . (stripPrefix "up ")]

update :: State -> Command -> State
update (State h v) (Forward n) = State (h + n) v
update (State h v) (Down n) = State h (v + n)
update (State h v) (Up n) = State h (v - n)

stateProduct :: State -> Int
stateProduct (State h v) = h * v

main = do
  contents <- readFile "input.txt"
  print $ stateProduct $ foldl update (State 0 0) $ catMaybes $ map parseCommand $ lines contents
