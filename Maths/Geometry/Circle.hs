module Circle where

import GHC.Float
import Line
import Point

data Circle = Line Point2d Point2d | CenterRadius Point2d Double

instance Show Circle where
  show (CenterRadius p r) = "(x - " ++ show x ++ ")^2 + (y - " ++ show y ++ ")^2 = " ++ show (r * r)
    where
      (x, y) = case p of Point2d x y -> (x, y)
  show (Line p1 p2) = show (CenterRadius p r) where (p, r) = toCenterRadius (Line p1 p2)

toCenterRadius :: Circle -> (Point2d, Double)
toCenterRadius circle = (point, distance)
  where
    (point, distance) = do
      case circle of
        Line p1 p2 -> (centerOfGravity [p1, p2], dist p1 p2 / 2)
        CenterRadius point radius -> (point, radius)

substituteCircle :: Circle -> Point2d -> Double
substituteCircle circle point = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) - r * r
  where
    (x1, y1) = case point of Point2d x1 y1 -> (x1, y1)
    (x2, y2, r) = do
      (x2, y2, r)
      where
        (p, r) = toCenterRadius circle
        (x2, y2) = case p of Point2d x2 y2 -> (x2, y2)

pointWithinCircleTest :: Circle -> Point2d -> Bool
pointWithinCircleTest circle point = substituteCircle circle point <= 0

-- Interaction of line with circle

lineThroughCircleTest :: Circle -> Line2d -> Bool
lineThroughCircleTest circle line = linePointDist line center <= r
  where
    (center, r) = toCenterRadius circle

calcChordIntersection :: Circle -> Line2d -> (Point2d, Point2d)
calcChordIntersection circle line = (a, b)
  where
    (center, r) = toCenterRadius circle
    -- Chord middle point
    point = linePointDistIntersectionPoint line center
    (xMid, yMid) = case point of Point2d x y -> (x, y)
    chordDistBy2 = chordLength circle line / 2
    (aa, bb, _) = toCoeffValue line
    sqrtVal = sqrtDouble (aa * aa + bb * bb)
    a = do
      Point2d x y
      where
        x = xMid - chordDistBy2 * bb / sqrtVal
        y = yMid + chordDistBy2 * aa / sqrtVal
    b = do
      Point2d x y
      where
        x = xMid + chordDistBy2 * bb / sqrtVal
        y = yMid - chordDistBy2 * aa / sqrtVal

circleLineIntersection :: Circle -> Line2d -> (Point2d, Point2d)
circleLineIntersection circle line
  | isTangent circle line = (p, p)
  | lineThroughCircleTest circle line = calcChordIntersection circle line
  | otherwise = (Point2d (1 / 0) (1 / 0), Point2d (1 / 0) (1 / 0))
  where
    p = chordMidPointIntersection circle line

isTangent :: Circle -> Line2d -> Bool
isTangent circle line = linePointDist line center == r
  where
    (center, r) = toCenterRadius circle

chordMidPointIntersection :: Circle -> Line2d -> Point2d
chordMidPointIntersection circle line = chordMiddlePoint
  where
    (center, r) = toCenterRadius circle
    chordMiddlePoint = linePointDistIntersectionPoint line center

chordLength :: Circle -> Line2d -> Double
chordLength circle line
  | d < r = 2 * sqrtDouble (r * r - d * d)
  | otherwise = 0
  where
    (d, r) = do
      let (center, r) = toCenterRadius circle
      (linePointDist line center, r)
