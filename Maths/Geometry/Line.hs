module Line where

import Point
import Text.XHtml (base)

data Line2d = Line2d Point2d Point2d | Coeff2d Float Float Float

data Line3d = Line3d Point3d Point3d deriving (Show)

instance Show Line2d where
  show (Coeff2d a b c) = show a ++ "x" ++ " + " ++ show b ++ "y" ++ " + " ++ show c ++ " = 0"
  show (Line2d p q) = show (Coeff2d a b c) where (a, b, c) = toCoeffValue (Line2d p q)

-- Return coefficient of given line if defined by two points
toCoeffValue :: Line2d -> (Float, Float, Float)
toCoeffValue line = (a, b, c)
  where
    (a, b, c) = do
      case line of
        Coeff2d a b c -> (a, b, c)
        Line2d p1 p2 -> (y1 - y2, x2 - x1, y2 * x1 - x2 * y1)
          where
            (x1, y1) = case p1 of Point2d x1 y1 -> (x1, y1)
            (x2, y2) = case p2 of Point2d x2 y2 -> (x2, y2)

slope :: Line2d -> Float
slope line = -a / b
  where
    (a, b, c) = toCoeffValue line

toCoeff :: Line2d -> Line2d
toCoeff line = Coeff2d a1 b1 c1
  where
    (a1, b1, c1) = toCoeffValue line

intersection :: Line2d -> Line2d -> Point2d
intersection line1 line2 = do
  if a1 * b2 - a2 * b1 == 0 -- If slope is same, then they are parallel, send value as inf inf
    then Point2d (1 / 0) (1 / 0)
    else Point2d ((c2 * b1 - c1 * b2) / (a1 * b2 - a2 * b1)) ((c1 * a2 - a1 * c2) / (a1 * b2 - a2 * b1))
  where
    (a1, b1, c1) = toCoeffValue line1
    (a2, b2, c2) = toCoeffValue line2

areLinesParallel :: Line2d -> Line2d -> Bool
areLinesParallel line1 line2 = slope line1 == slope line2

areLinesPerpendicular :: Line2d -> Line2d -> Bool
areLinesPerpendicular line1 line2 = (slope line1 * slope line2 + 1) < 1e-9 -- within precision

areLinesSame :: Line2d -> Line2d -> Bool
areLinesSame line1 line2 = slope line1 == slope line2 && c1 == c2
  where
    (_, _, c1) = toCoeffValue line1
    (_, _, c2) = toCoeffValue line2

neq :: Line2d -> Line2d -> Bool
neq line1 line2 = (a2 / a1 /= b2 / b1) || (b2 / b1 /= c2 / c1)
  where
    (a1, b1, c1) = toCoeffValue line1
    (a2, b2, c2) = toCoeffValue line2

perpendicularLine :: Line2d -> Point2d -> Line2d
perpendicularLine line throughPoint = Coeff2d b (-a) (a * y - b * x)
  where
    (a, b, _) = toCoeffValue line
    (x, y) = case throughPoint of Point2d x y -> (x, y)

angleBetweenLines :: Line2d -> Line2d -> Float
angleBetweenLines line1 line2 =
  if (m1 * m2 + 1) < 1e-9
    then pi / 2
    else atan ((m1 - m2) / (1 + m1 * m2))
  where
    m1 = slope line1
    m2 = slope line2

testForPoint :: Line2d -> Point2d -> Bool
testForPoint line point = a * x + b * y + c < 1e-7
  where
    (a, b, c) = toCoeffValue line
    (x, y) = case point of Point2d x y -> (x, y)

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
