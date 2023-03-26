import GHC.Float (sqrtFloat)

data Point2d = Point2d Float Float deriving (Show)

data Point3d = Point3d Float Float Float deriving (Show)

data Line2d = Line2d Point2d Point2d | Coeff2d Float Float Float deriving (Show)

data Line3d = Line3d Point3d Point3d deriving (Show)

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

add :: Point2d -> Point2d -> Point2d
add a b = Point2d (x1 + x2) (y1 + y2)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)

sub :: Point2d -> Point2d -> Point2d
sub a b = Point2d (x1 - x2) (y1 - y2)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)

mul :: Point2d -> Float -> Point2d
mul a b = Point2d (x1 * b) (y1 * b)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)

div :: Point2d -> Float -> Point2d
div a b = Point2d (x1 / b) (y1 / b)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)

dot :: Point2d -> Point2d -> Float
dot a b = x1 * x2 + y1 * y2
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)

dot3d :: Point3d -> Point3d -> Float
dot3d a b = x1 * x2 + y1 * y2 + z1 * z2
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)

cross3d :: Point3d -> Point3d -> Point3d
cross3d a b = Point3d (y1 * z2 - z1 * y2) (z1 * x2 - x1 * z2) (x1 * y2 - y1 * x2)
  where
    (x1, y1, z1) = case a of Point3d x1 y1 z1 -> (x1, y1, z1)
    (x2, y2, z2) = case b of Point3d x2 y2 z2 -> (x2, y2, z2)

normal :: Point2d -> Float
normal p = dot p p

normal3d :: Point3d -> Float
normal3d p = dot3d p p

abs3d :: Point3d -> Float
abs3d p = sqrtFloat (dot3d p p)

angleX :: Point2d -> Float
angleX p = atan (y / x)
  where
    (x, y) = case p of Point2d x y -> (x, y)

angleY :: Point2d -> Float
angleY p = angleX p - pi / 2

slope2Points :: Point2d -> Point2d -> Float
slope2Points a b = (y2 - y1) / (x2 - x1)
  where
    (x1, y1) = case a of Point2d x1 y1 -> (x1, y1)
    (x2, y2) = case b of Point2d x2 y2 -> (x2, y2)

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

main = do
  -- define line using two points
  let p = Line2d (Point2d 10.0 5) (Point2d 5 2.5)
  let q = Line2d (Point2d 10.0 5) (Point2d 5 (-2.5))
  print (show (slope p) ++ " " ++ show (slope q))
  -- define lines using coefficients
  let r = Coeff2d 10 5 1
  let s = Coeff2d 23 44 133
  -- example of parallel line
  let t = Coeff2d 2 3 15
  let u = Coeff2d 4 6 10
  print (intersection p q)
  print (intersection r s)
  print (intersection t u)
  return 0