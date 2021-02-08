module SpaceAge (Planet (..), ageOn) where

data Planet
  = Mercury
  | Venus
  | Earth
  | Mars
  | Jupiter
  | Saturn
  | Uranus
  | Neptune

type Seconds = Float

type Years = Float

ageOn :: Planet -> Seconds -> Years
ageOn Earth = (/ 31557600)
ageOn planet =
  ( /
      case planet of
        Mercury -> 0.2408467 -- Then recursively call ageOn to with Earth as the base seconds
        Venus -> 0.61519726
        Mars -> 1.8808158
        Jupiter -> 11.862615
        Saturn -> 29.447498
        Uranus -> 84.016846
        Neptune -> 164.79132
        _ -> 1 -- not possible
  )
    . ageOn Earth -- seconds has been η-reduced, which was possible because seconds was at the end because the Division Operator is a prefix function when put into brackets. Awesome!

{-
ageOn :: Planet -> Float -> Float
ageOn planet seconds =
  case planet of
    Earth   -> seconds             / 31557600     -- Convert seconds to earth years
    Mercury -> ageOn Earth seconds / 0.2408467    -- Then recursively call ageOn to with Earth as the base seconds
    Venus   -> ageOn Earth seconds / 0.61519726
    Mars    -> ageOn Earth seconds / 1.8808158
    Jupiter -> ageOn Earth seconds / 11.862615
    Saturn  -> ageOn Earth seconds / 29.447498
    Uranus  -> ageOn Earth seconds / 84.016846
    Neptune -> ageOn Earth seconds / 164.79132
-}

-- ageOn :: Planet -> Float -> Float
-- ageOn Earth   = (/ 31557600)
-- ageOn Mercury = (/ 0.2408467)  . ageOn Earth
-- ageOn Venus   = (/ 0.61519726) . ageOn Earth
-- ageOn Mars    = (/ 1.8808158)  . ageOn Earth
-- ageOn Jupiter = (/ 11.862615)  . ageOn Earth
-- ageOn Saturn  = (/ 29.447498)  . ageOn Earth
-- ageOn Uranus  = (/ 84.016846)  . ageOn Earth
-- ageOn Neptune = (/ 164.79132)  . ageOn Earth

-- ageOn :: Planet -> Float -> Float
-- ageOn planet seconds = seconds / earth_seconds / orbital_factor
--   where
--     earth_seconds = 31557600 -- 365.25 days * 24 h * 60 min * 60 sec
--     orbital_factor = case planet of -- orbit factor (fraction of earth orbiting time = 1 year)
--       Mercury -> 0.2408467
--       Venus -> 0.61519726
--       Earth -> 1
--       Mars -> 1.8808158
--       Jupiter -> 11.862615
--       Saturn -> 29.447498
--       Uranus -> 84.016846
--       Neptune -> 164.79132
