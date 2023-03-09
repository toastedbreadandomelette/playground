struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn sq_abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }
}

fn main() {
    println!("Hello");
}
