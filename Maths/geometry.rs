#[derive(Debug)]
struct Point<T> {
    pub x: T,
    pub y: T,
}

trait Sqrt {
    fn _sqrt(&self) -> f64;
}

macro_rules! impl_trait_sqrt {
    ($type: ident) => {
        impl Sqrt for $type {
            fn _sqrt(&self) -> f64 {
                (*self as f64).sqrt()
            }
        }
    };
}

impl_trait_sqrt!(f32);
impl_trait_sqrt!(f64);
impl_trait_sqrt!(i8);
impl_trait_sqrt!(i16);
impl_trait_sqrt!(i32);
impl_trait_sqrt!(i64);
impl_trait_sqrt!(u8);
impl_trait_sqrt!(u16);
impl_trait_sqrt!(u32);
impl_trait_sqrt!(u64);

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Sqrt> Point<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y)._sqrt()
    }

    pub fn sq_abs(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Point<T>> for Point<T> {
    type Output = Self;
    fn add(self, rhs: Point<T>) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p = Point::new(3, 4);
    println!("{:?}", (p + Point::new(6, 8)).abs());
}
