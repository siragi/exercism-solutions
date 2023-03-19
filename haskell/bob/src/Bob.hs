{-# LANGUAGE OverloadedStrings #-}

module Bob (responseFor) where

import Data.Char
-- import Data.List
import Data.Text.Lazy (Text)
import qualified Data.Text.Lazy as T

-- import Data.Text (Text)
-- import qualified Data.Text as T

responseFor :: Text -> Text
responseFor xs =
  responseForStripped (T.strip xs)
  where
    {- strip = T.reverse . T.dropWhile isSpace . T.reverse . T.dropWhile isSpace -}
    responseForStripped text
      | text == "" = "Fine. Be that way!"
      | T.any isLetter text && T.last text == '?' && T.all isUpper (T.filter isLetter text) = "Calm down, I know what I'm doing!" -- Yelling a question
      | T.last text == '?' = "Sure." -- Question
      -- | T.any isLetter text && T.all isUpper (T.filter isLetter text) = "Whoa, chill out!" -- Yelling
      | T.any isLetter text && not (T.any isLower text) = "Whoa, chill out!" -- Yelling
      -- | all isUpper [c | c <- text, isAlpha c] = "Whoa, chill out!" -- Yelling
      | otherwise = "Whatever."
