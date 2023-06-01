module Point where

import GHC.Float (sqrtDouble)

-- Point in a 2-dimensional field (plane)
data Point2d = Point2d Double Double

instance Show Point2d where
  show (Point2d x y) = "(" ++ show x ++ ", " ++ show y ++ ")"

unpackCoord :: Point2d -> (Double, Double)
unpackCoord point = (x, y)
  where
    (x, y) = case point of Point2d x y -> (x, y)
{-# INLINEABLE unpackCoord #-}

add :: Point2d -> Point2d -> Point2d
add a b = Point2d (x1 + x2) (y1 + y2)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE add #-}

sub :: Point2d -> Point2d -> Point2d
sub a b = Point2d (x1 - x2) (y1 - y2)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE sub #-}

mul :: Point2d -> Double -> Point2d
mul a b = Point2d (x1 * b) (y1 * b)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
{-# INLINEABLE mul #-}

div2d :: Point2d -> Double -> Point2d
div2d a b = Point2d (x1 / b) (y1 / b)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
{-# INLINEABLE div2d #-}

dot :: Point2d -> Point2d -> Double
dot a b = x1 * x2 + y1 * y2
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE dot #-}

normal :: Point2d -> Double
normal p = dot p p
{-# INLINEABLE normal #-}

dist :: Point2d -> Point2d -> Double
dist point1 point2 = sqrtDouble ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))
  where
    (x1, y1) = case point1 of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case point2 of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE dist #-}

angleX :: Point2d -> Double
angleX p = atan (y / x)
  where
    (x, y) = case p of Point2d x y -> (x, y)
{-# INLINEABLE angleX #-}

angleY :: Point2d -> Double
angleY p = angleX p - pi / 2
{-# INLINEABLE angleY #-}

slope2Points :: Point2d -> Point2d -> Double
slope2Points a b = (y2 - y1) / (x2 - x1)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)
{-# INLINEABLE slope2Points #-}

xVal :: Point2d -> Double
xVal point = x where x = case point of Point2d x _ -> x
{-# INLINEABLE xVal #-}

yVal :: Point2d -> Double
yVal point = y where y = case point of Point2d _ y -> y
{-# INLINEABLE yVal #-}

centerOfGravity :: [Point2d] -> Point2d
centerOfGravity list = div2d (foldr add (Point2d 0 0) list) listLength
  where
    listLength = fromIntegral (length list)
{-# INLINEABLE centerOfGravity #-}

weightedCenterOfGravity :: [(Double, Point2d)] -> Point2d
weightedCenterOfGravity list = Point2d (xsum / div) (ysum / div)
  where
    xsum = sum [xVal p * weight | (weight, p) <- list]
    ysum = sum [yVal p * weight | (weight, p) <- list]
    div = sum [weight | (weight, _) <- list]
{-# INLINEABLE weightedCenterOfGravity #-}

collinearity :: Point2d -> Point2d -> Point2d -> Bool
collinearity point1 point2 point3 = slope2Points point1 point2 == slope2Points point2 point3
{-# INLINEABLE collinearity #-}

perpendicularity :: Point2d -> Point2d -> Point2d -> Bool
perpendicularity point1 point2 point3 = slope2Points point1 point2 * slope2Points point2 point3 + 1 < 1e-6
{-# INLINEABLE perpendicularity #-}

-- Point in a 3-dimensional field (space)
data Point3d = Point3d Double Double Double

instance Show Point3d where
  show (Point3d x y z) = "(" ++ show x ++ ", " ++ show y ++ ", " ++ show z ++ ")"

add3d :: Point3d -> Point3d -> Point3d
add3d a b = Point3d x1 y1 z1
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)
{-# INLINEABLE add3d #-}

sub3d :: Point3d -> Point3d -> Point3d
sub3d a b = Point3d (x1 - x2) (y1 - y2) (z1 - z2)
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)
{-# INLINEABLE sub3d #-}

mul3d :: Point3d -> Double -> Point3d
mul3d a b = Point3d (x1 * b) (y1 * b) (z1 * b)
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
{-# INLINEABLE mul3d #-}

div3d :: Point3d -> Double -> Point3d
div3d a b = Point3d (x1 / b) (y1 / b) (z1 / b)
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
{-# INLINEABLE div3d #-}

dot3d :: Point3d -> Point3d -> Double
dot3d a b = x1 * x2 + y1 * y2 + z1 * z2
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)
{-# INLINEABLE dot3d #-}

cross3d :: Point3d -> Point3d -> Point3d
cross3d a b = Point3d (y1 * z2 - z1 * y2) (z1 * x2 - x1 * z2) (x1 * y2 - y1 * x2)
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)
{-# INLINEABLE cross3d #-}

xVal3d :: Point3d -> Double
xVal3d point = x where x = case point of Point3d x _ _ -> x
{-# INLINEABLE xVal3d #-}

yVal3d :: Point3d -> Double
yVal3d point = y where y = case point of Point3d _ y _ -> y
{-# INLINEABLE yVal3d #-}

zVal3d :: Point3d -> Double
zVal3d point = z where z = case point of Point3d _ _ z -> z
{-# INLINEABLE zVal3d #-}

centerOfGravity3d :: [Point3d] -> Point3d
centerOfGravity3d list = div3d (foldr add3d (Point3d 0 0 0) list) listLength
  where
    listLength = fromIntegral (length list)
{-# INLINEABLE centerOfGravity3d #-}

weightedCenterOfGravity3d :: [(Double, Point3d)] -> Point3d
weightedCenterOfGravity3d list = Point3d (xsum / div) (ysum / div) (zsum / div)
  where
    xsum = sum [xVal3d p * weight | (weight, p) <- list]
    ysum = sum [yVal3d p * weight | (weight, p) <- list]
    zsum = sum [zVal3d p * weight | (weight, p) <- list]
    div = sum [weight | (weight, _) <- list]
{-# INLINEABLE weightedCenterOfGravity3d #-}

normal3d :: Point3d -> Double
normal3d p = dot3d p p
{-# INLINEABLE normal3d #-}

abs3d :: Point3d -> Double
abs3d p = sqrtDouble (dot3d p p)
{-# INLINEABLE abs3d #-}

angleX3d :: Point3d -> Double
angleX3d p = acos (x / abs3d p)
  where
    x = case p of Point3d x _ _ -> x
{-# INLINEABLE angleX3d #-}

angleY3d :: Point3d -> Double
angleY3d p = acos (y / abs3d p)
  where
    y = case p of Point3d _ y _ -> y
{-# INLINEABLE angleY3d #-}

angleZ3d :: Point3d -> Double
angleZ3d p = acos (z / abs3d p)
  where
    z = case p of Point3d _ _ z -> z
{-# INLINEABLE angleZ3d #-}