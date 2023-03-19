module DNA (nucleotideCounts, Nucleotide (..)) where

import Data.Map (Map)
import qualified Data.Map as Map

data Nucleotide = A | C | G | T deriving (Eq, Ord, Show)

dna = [A, C, G, T]

nucleotideCounts :: String -> Either String (Map Nucleotide Int)
nucleotideCounts = mapM count
  where
    count :: Char -> Either Char (Map Nucleotide Int)
    count nucleotide
        | nucleotide `elem` dna = Right (insertWith (+ 1) nucleotide)
        | otherwise Left nucleotide