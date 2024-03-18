main = do
    print $ twoSum [2,7,11,15] 9
    print $ twoSum [3,2,4] 6
    print $ twoSum [3,3] 6

twoSum :: [Int] -> Int -> [(Int, Int)]
twoSum nums target = zip [0..] nums  
