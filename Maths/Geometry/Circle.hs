module Circle where

import Line
import Point

data Circle = Line Point2d Point2d | CenterRadius Point2d Double

instance Show Circle where
  show (CenterRadius p r) = "(x - " ++ show x ++ ")^2 + (y - " ++ show y ++ ")^2 = " ++ show r ++ "^2"
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