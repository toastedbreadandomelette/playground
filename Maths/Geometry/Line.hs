module Line where

import GHC.Float (sqrtDouble)
import Point

data Line2d = Line2d Point2d Point2d | Coeff2d Double Double Double

data Line3d = Line3d Point3d Point3d deriving (Show)

instance Show Line2d where
  show (Coeff2d a b c) = show a ++ "x" ++ " + " ++ show b ++ "y" ++ " + " ++ show c ++ " = 0"
  show (Line2d p q) = show (Coeff2d a b c) where (a, b, c) = toCoeffValue (Line2d p q)

-- Return coefficient of given line if defined by two points
toCoeffValue :: Line2d -> (Double, Double, Double)
toCoeffValue line = (a, b, c)
  where
    (a, b, c) = do
      case line of
        Coeff2d a b c -> (a, b, c)
        Line2d p1 p2 -> (y1 - y2, x2 - x1, y2 * x1 - x2 * y1)
          where
            (x1, y1) = case p1 of Point2d x1 y1 -> (x1, y1)
            (x2, y2) = case p2 of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE toCoeffValue #-}

slope :: Line2d -> Double
slope line = -a / b
  where
    (a, b, c) = toCoeffValue line
{-# INLINEABLE slope #-}

toCoeff :: Line2d -> Line2d
toCoeff line = Coeff2d a1 b1 c1
  where
    (a1, b1, c1) = toCoeffValue line
{-# INLINEABLE toCoeff #-}

intersection :: Line2d -> Line2d -> Point2d
intersection line1 line2
  | a1 * b2 - a2 * b1 == 0 = Point2d (1 / 0) (1 / 0) -- If slope is same, then they are parallel, send value as inf inf
  | otherwise = Point2d ((c2 * b1 - c1 * b2) / (a1 * b2 - a2 * b1)) ((c1 * a2 - a1 * c2) / (a1 * b2 - a2 * b1))
  where
    (a1, b1, c1) = toCoeffValue line1
    (a2, b2, c2) = toCoeffValue line2

areLinesParallel :: Line2d -> Line2d -> Bool
areLinesParallel line1 line2 = slope line1 == slope line2
{-# INLINEABLE areLinesParallel #-}

areLinesPerpendicular :: Line2d -> Line2d -> Bool
areLinesPerpendicular line1 line2 = (slope line1 * slope line2 + 1) < 1e-9 -- within precision
{-# INLINEABLE areLinesPerpendicular #-}

areLinesSame :: Line2d -> Line2d -> Bool
areLinesSame line1 line2 = slope line1 == slope line2 && c1 == c2
  where
    (_, _, c1) = toCoeffValue line1
    (_, _, c2) = toCoeffValue line2
{-# INLINEABLE areLinesSame #-}

neq :: Line2d -> Line2d -> Bool
neq line1 line2 = (a2 / a1 /= b2 / b1) || (b2 / b1 /= c2 / c1)
  where
    (a1, b1, c1) = toCoeffValue line1
    (a2, b2, c2) = toCoeffValue line2
{-# INLINEABLE neq #-}

perpendicularLine :: Line2d -> Point2d -> Line2d
perpendicularLine line throughPoint = Coeff2d b (-a) (a * y - b * x)
  where
    (a, b, _) = toCoeffValue line
    (x, y) = case throughPoint of Point2d x y -> (x, y)
{-# INLINEABLE perpendicularLine #-}

angleBetweenLines :: Line2d -> Line2d -> Double
angleBetweenLines line1 line2
  | (m1 * m2 + 1) < 1e-9 = pi / 2
  | otherwise = atan ((m1 - m2) / (1 + m1 * m2))
  where
    m1 = slope line1
    m2 = slope line2

substitute :: Line2d -> Point2d -> Double
substitute line point = a * x + b * y + c
  where
    (x, y) = case point of Point2d x y -> (x, y)
    (a, b, c) = toCoeffValue line
{-# INLINEABLE substitute #-}

testForPoint :: Line2d -> Point2d -> Bool
testForPoint line point = a * x + b * y + c < 1e-9
  where
    (a, b, c) = toCoeffValue line
    (x, y) = case point of Point2d x y -> (x, y)
{-# INLINEABLE testForPoint #-}

linePointDist :: Line2d -> Point2d -> Double
linePointDist line point = abs (substitute line point / sqrtDouble (a * a + b * b))
  where
    (a, b, _) = toCoeffValue line
{-# INLINEABLE linePointDist #-}

linePointDistIntersectionPoint :: Line2d -> Point2d -> Point2d
linePointDistIntersectionPoint line point = intersection line (Coeff2d (-b) a (b * x - a * y))
  where
    (a, b, _) = toCoeffValue line
    (x, y) = case point of Point2d x y -> (x, y)
{-# INLINEABLE linePointDistIntersectionPoint #-}

parallelLineDist :: Line2d -> Line2d -> Double
parallelLineDist line1 line2
  | slope line1 == slope line2 = abs (c1 - c2) / sqrt (a * a + b * b)
  | otherwise = 1 / 0
  where
    (a, b, c1) = toCoeffValue line1
    (_, _, c2) = toCoeffValue line2
{-# INLINEABLE parallelLineDist #-}

-- Generate all intersection points
-- Returns list of two lines and point of intersection
allPointIntersection :: [Line2d] -> [(Line2d, Line2d, Point2d)]
allPointIntersection lineList =
  [ (toCoeff x, toCoeff y, intersection x y)
    | (i, x) <- zip [0 ..] lineList,
      (j, y) <- zip [0 ..] lineList,
      neq x y, -- x and y should not be included
      i < j -- no repetition
  ]
{-# INLINEABLE allPointIntersection #-}