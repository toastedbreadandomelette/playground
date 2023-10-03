use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
pub const PI: f64 = core::f64::consts::PI;

// Complex number for computing.
#[derive(Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub img: f64,
}

pub trait Number {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for usize {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for f32 {}
impl Number for f64 {}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            real: -self.real,
            img: -self.img,
        }
    }
}

impl core::fmt::Debug for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.real == 0.0 && self.img == 0.0 {
            f.write_str("0")
        } else if self.real == 0.0 {
            f.write_str(format!("{:.4}i", self.img).as_str())
        } else if self.img == 0.0 {
            f.write_str(format!("{:.4}", self.real).as_str())
        } else if self.img > 0.0 {
            f.write_str(format!("{:.4}+{:.4}i", self.real, self.img).as_str())
        } else {
            f.write_str(format!("{:.4}{:.4}i", self.real, self.img).as_str())
        }
    }
}

impl AddAssign<&Complex> for Complex {
    fn add_assign(&mut self, b: &Self) {
        self.real += b.real;
        self.img += b.img;
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, b: Self) {
        self.real += b.real;
        self.img += b.img;
    }
}

impl Sub<&Complex> for Complex {
    type Output = Self;

    fn sub(self, b: &Complex) -> Self {
        Self {
            real: self.real - b.real,
            img: self.img - b.img,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Self;

    fn sub(self, b: Complex) -> Self {
        Self {
            real: self.real - b.real,
            img: self.img - b.img,
        }
    }
}

impl SubAssign<&Complex> for Complex {
    fn sub_assign(&mut self, b: &Self) {
        self.real -= b.real;
        self.img -= b.img;
    }
}

impl<T> SubAssign<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    fn sub_assign(&mut self, b: T) {
        self.real -= b.into();
    }
}

impl<T> Sub<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Self;
    fn sub(self, b: T) -> Self {
        Self {
            real: self.real - b.into(),
            img: self.img,
        }
    }
}

impl<T> Add<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Complex;
    fn add(self, b: T) -> Complex {
        Complex {
            real: self.real + b.into(),
            img: self.img + b.into(),
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Self;
    fn add(self, b: Complex) -> Self {
        Self {
            real: self.real + b.real,
            img: self.img + b.img,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    fn add_assign(&mut self, b: T) {
        self.real += b.into();
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, b: Complex) -> Complex {
        Complex {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + b.img * self.real,
        }
    }
}

impl<T> Mul<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Complex;
    fn mul(self, b: T) -> Complex {
        Complex {
            real: self.real * b.into(),
            img: self.img * b.into(),
        }
    }
}

impl MulAssign<&Complex> for Complex {
    fn mul_assign(&mut self, b: &Self) {
        *self = Self {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + self.real * b.img,
        }
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, b: Self) {
        *self = Self {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + self.real * b.img,
        }
    }
}

impl<T> MulAssign<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    fn mul_assign(&mut self, b: T) {
        self.real *= b.into();
        self.img *= b.into();
    }
}

impl Div<&Complex> for Complex {
    type Output = Self;
    fn div(self, b: &Complex) -> Self {
        let abs = b.abs_sq();
        Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Self;
    fn div(self, b: Complex) -> Self {
        let abs = b.abs_sq();
        Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl<T> Div<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Complex;
    fn div(self, b: T) -> Complex {
        Complex {
            real: self.real / b.into(),
            img: self.img / b.into(),
        }
    }
}

impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, b: Self) {
        let abs = b.abs_sq();
        *self = Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl DivAssign<&Complex> for Complex {
    fn div_assign(&mut self, b: &Self) {
        let abs = b.abs_sq();
        *self = Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl<T> DivAssign<T> for Complex
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    fn div_assign(&mut self, b: T) {
        self.real /= b.into();
        self.img /= b.into();
    }
}

impl core::fmt::Display for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.real == 0.0 && self.img == 0.0 {
            f.write_str("0")
        } else if self.real == 0.0 {
            f.write_str(format!("{:.4}i", self.img).as_str())
        } else if self.img == 0.0 {
            f.write_str(format!("{:.4}", self.real).as_str())
        } else if self.img > 0.0 {
            f.write_str(format!("{:.4}+{:.4}i", self.real, self.img).as_str())
        } else {
            f.write_str(format!("{:.4}{:.4}i", self.real, self.img).as_str())
        }
    }
}

macro_rules! impl_op_for_type {
    ($type: ident) => {
        impl core::convert::From<$type> for Complex {
            fn from(item: $type) -> Complex {
                Complex {
                    real: (item as f64),
                    img: 0.0,
                }
            }
        }

        impl core::convert::From<Complex> for $type {
            fn from(item: Complex) -> Self {
                item.real as $type
            }
        }

        impl Add<Complex> for $type {
            type Output = Complex;
            fn add(self, b: Complex) -> Complex {
                Complex {
                    real: b.real + (self as f64),
                    img: b.img,
                }
            }
        }

        impl AddAssign<Complex> for $type {
            fn add_assign(&mut self, b: Complex) {
                *self += (b.real as $type);
            }
        }

        impl Sub<Complex> for $type {
            type Output = Complex;
            fn sub(self, b: Complex) -> Complex {
                Complex {
                    real: -b.real + (self as f64),
                    img: -b.img,
                }
            }
        }

        impl SubAssign<Complex> for $type {
            fn sub_assign(&mut self, b: Complex) {
                *self -= (b.real as $type);
            }
        }

        impl Mul<Complex> for $type {
            type Output = Complex;
            fn mul(self, b: Complex) -> Complex {
                Complex {
                    real: b.real * (self as f64),
                    img: b.img * (self as f64),
                }
            }
        }

        impl MulAssign<Complex> for $type {
            fn mul_assign(&mut self, b: Complex) {
                *self *= (b.real as $type);
            }
        }

        impl Div<Complex> for $type {
            type Output = Complex;
            fn div(self, b: Complex) -> Complex {
                let slf = (self as f64);
                let abs = b.abs_sq();
                Complex {
                    real: (slf * b.real) / abs,
                    img: (slf * b.img) / abs,
                }
            }
        }
    };
}

impl_op_for_type!(f64);
impl_op_for_type!(f32);
impl_op_for_type!(i8);
impl_op_for_type!(i16);
impl_op_for_type!(i32);
impl_op_for_type!(i64);
impl_op_for_type!(i128);
impl_op_for_type!(u8);
impl_op_for_type!(u16);
impl_op_for_type!(u32);
impl_op_for_type!(u64);
impl_op_for_type!(u128);

impl Complex {
    #[inline(always)]
    pub fn new(r: f64, i: f64) -> Self {
        Self { real: r, img: i }
    }

    pub fn zero() -> Self {
        Self {
            real: 0.0,
            img: 0.0,
        }
    }

    #[inline(always)]
    pub fn abs_sq(self) -> f64 {
        self.real * self.real + self.img * self.img
    }

    #[inline(always)]
    pub fn abs(self) -> f64 {
        self.abs_sq().sqrt()
    }

    pub fn pow(self, mut power: u32) -> Self {
        match power {
            0 => Self {
                real: 1.0,
                img: 0.0,
            },
            1 => self,
            2 => self * self,
            3 => self * self * self,
            _ => {
                let mut result = Self {
                    real: 1.0,
                    img: 0.0,
                };
                let mut mul = self;
                while power > 0 {
                    if power & 1 == 1 {
                        result *= mul;
                    }
                    mul *= mul;
                    power >>= 1;
                }
                result
            }
        }
    }

    pub fn conjugate(self) -> Self {
        Self {
            real: self.real,
            img: -self.img,
        }
    }
}
