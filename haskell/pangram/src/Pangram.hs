module Pangram (isPangram) where

import Data.Char
import Data.List (intersect, nub, sort)

isPangram :: String -> Bool
isPangram = (['a' .. 'z'] ==) . sort . nub . map toLower . filter isAscii . filter isLetter

-- isPangram text = ['a' .. 'z'] == (sort . nub . map toLower . filter isAscii . filter isLetter) text -- adapted from https://exercism.io/tracks/haskell/exercises/pangram/solutions/9aec18fb5f38426f80c2b94acbfc312e

-- isPangram = flip all ['a' .. 'z'] . flip elem . map toLower

-- isPangram = (`all` ['a' .. 'z']) . flip elem . map toLower
-- isPangram text = ((`all` ['a' .. 'z']) . (flip elem . map toLower)) text
-- isPangram text = (`all` ['a' .. 'z']) ((flip elem . map toLower) text) -- f(g x)
-- isPangram text = (flip elem . map toLower) text `all` ['a' .. 'z']
-- isPangram text = all ((flip elem . map toLower) text) ['a' .. 'z']
-- isPangram text = all (`elem` map toLower text) ['a' .. 'z']
-- isPangram text = all (\char -> char `elem` map toLower text) ['a' .. 'z']

-- isPangram = and . flip map ['a' .. 'z'] . flip elem . map toLower

-- isPangram = and . (<$> ['a' .. 'z']) . flip elem . map toLower -- <$> means infix `fmap` and fmap with lists means map, so flip map list does the same as (<$> list)

{- isPangram = and . (<$> ['a' .. 'z']) . isElem
  where
    isElem text = (`elem` map toLower text) -}

{- isElem :: [Char] -> Char -> Bool
isElem text = (`elem` map toLower text) -}
-- isPangram = and . (<$> ['a' .. 'z']) . isElem
-- isPangram text = ((and . (<$> ['a' .. 'z'])) . (isElem)) text
-- isPangram text = (and . (<$> ['a' .. 'z'])) (isElem text)
-- isPangram text = and ((isElem text) <$> ['a' .. 'z'])
-- isPangram text = and ((<$>) (isElem text) ['a' .. 'z'])
-- isPangram text = and ((<$>) (`elem` map toLower text) ['a' .. 'z'])
-- isPangram text = and ((`elem` map toLower text) <$> ['a' .. 'z'])

-- "there should be 26 . elements in . the unique list of . elements filtered on (being between a & z) . from the input where each element is transformed to lower case
-- isPangram = (== 26) . length . nub . filter (`elem` ['a' .. 'z']) . map toLower

-- isPangram = (26 ==) . length . nub . intersect ['a' .. 'z'] . map toLower

-- isPangram text = 26 == length (nub $ ['a' .. 'z'] `intersect` map toLower text)
-- isPangram text = 26 == length (nub (['a' .. 'z'] `intersect` map toLower text))
-- isPangram text = length (nub (['a' .. 'z'] `intersect` map toLower text)) == 26
