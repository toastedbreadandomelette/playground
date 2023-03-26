import Line
import Point

main = do
  -- define line using two points
  let p = Line2d (Point2d 10.0 5) (Point2d 5 2.5)
  let q = Line2d (Point2d 10.0 5) (Point2d 5 (-2.5))
  -- define lines using coefficients
  let r = Coeff2d 10 5 1
  let s = Coeff2d 23 44 133
  -- example of parallel line
  let t = Coeff2d 2 3 15
  let u = Coeff2d 4 6 10

  -- print (allPointIntersection [p, q, r, s, t, u])
  let ans = intersection p q
  print q
  print (slope q)
  print p
  print (slope p)
  let perp = perpendicularLine q (Point2d 10.0 5)

  print (angleBetweenLines p q)

  return 0