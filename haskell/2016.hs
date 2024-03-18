maximumDifference
    = foldl max (-1)
    . filter (/=0)
    . (zipWith (-) <*> scanl1 min)


main = do
    print $ maximumDifference [7, 1, 5, 4]
    print $ maximumDifference [9, 4, 3, 2]
    print $ maximumDifference [1, 5, 2, 10]
