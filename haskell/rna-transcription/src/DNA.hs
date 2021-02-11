module DNA (
    toRNA,
) where

import Data.List
import Data.Set (Set)
import qualified Data.Set as Set

type Nucleotide = Char
type Dna = [Nucleotide]
type Rna = [Nucleotide]

dnaList :: Dna
dnaList = "GCTA"

rnaList :: Rna
rnaList = "CGAU"

toRNA :: Dna -> Either Nucleotide Rna
toRNA [] = Right ""
toRNA dna
    --  | all (`elem` dnaList) dna = Right (transscribe dna)
    | set dna ⊆ set dnaList = Right (transscribe dna)
    | otherwise = Left (head nonDna)
  where
    nonDna = dropWhile (`elem` dnaList) dna

transscribe :: [Nucleotide] -> [Nucleotide]
transscribe [] = []
transscribe (n : ns) =
    let Just index = elemIndex n dnaList
        r = rnaList !! index
     in r : transscribe ns

(⊆) :: (Ord a) => Set a -> Set a -> Bool
(⊆) = Set.isSubsetOf

set :: (Ord a) => [a] -> Set a
set = Set.fromList

-- adapted from https://exercism.io/tracks/haskell/exercises/rna-transcription/solutions/0aecf387411c429cb7a724217aa40a51

{- toRNA :: String -> Either Char String
toRNA = mapM transcription
  where
    transcription :: Char -> Either Char Char
    transcription str = case str of
        'G' -> Right 'C'
        'C' -> Right 'G'
        'T' -> Right 'A'
        'A' -> Right 'U'
        _ -> Left str -}

-- adapted from https://exercism.io/tracks/haskell/exercises/rna-transcription/solutions/8be9160d453249b781ec020da8095efb

{- import Control.Applicative (liftA2)
import Data.Set as Set

toRNA :: String -> Either Char String
toRNA "" = Right ""
toRNA (x : xs)
    | x == 'G' = (:) <$> pure 'C' <*> toRNA xs -- <$> is fmap
    | x == 'C' = fmap ('G' :) (toRNA xs)
    | x == 'T' = fmap ((:) 'A') (toRNA xs)
    | x == 'A' = liftA2 (:) (pure 'U') (toRNA xs) -- liftA2 is <$> (fmap) and <*> (say 'apply') --> see first line
    | otherwise = Left x -}