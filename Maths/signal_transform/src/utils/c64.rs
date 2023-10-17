use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
pub const PI: f64 = core::f64::consts::PI;

// C64 number for computing.
#[derive(Copy, Clone, PartialEq)]
pub struct C64 {
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

impl Neg for C64 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            img: -self.img,
        }
    }
}

impl core::fmt::Debug for C64 {
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

impl AddAssign<&C64> for C64 {
    #[inline(always)]
    fn add_assign(&mut self, b: &Self) {
        self.real += b.real;
        self.img += b.img;
    }
}

impl AddAssign for C64 {
    #[inline(always)]
    fn add_assign(&mut self, b: Self) {
        self.real += b.real;
        self.img += b.img;
    }
}

impl Sub<&C64> for C64 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, b: &Self) -> Self {
        Self {
            real: self.real - b.real,
            img: self.img - b.img,
        }
    }
}

impl Sub<C64> for C64 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, b: Self) -> Self {
        Self {
            real: self.real - b.real,
            img: self.img - b.img,
        }
    }
}

impl SubAssign<&C64> for C64 {
    #[inline(always)]
    fn sub_assign(&mut self, b: &Self) {
        self.real -= b.real;
        self.img -= b.img;
    }
}

impl<T> SubAssign<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    #[inline(always)]
    fn sub_assign(&mut self, b: T) {
        self.real -= b.into();
    }
}

impl<T> Sub<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, b: T) -> Self {
        Self {
            real: self.real - b.into(),
            img: self.img,
        }
    }
}

impl<T> Add<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, b: T) -> Self {
        Self {
            real: self.real + b.into(),
            img: self.img,
        }
    }
}

impl Add<C64> for C64 {
    type Output = Self;
    #[inline(always)]
    fn add(self, b: C64) -> Self {
        Self {
            real: self.real + b.real,
            img: self.img + b.img,
        }
    }
}

impl<T> AddAssign<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    #[inline(always)]
    fn add_assign(&mut self, b: T) {
        self.real += b.into();
    }
}

impl Mul<C64> for C64 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, b: Self) -> Self {
        Self {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + b.img * self.real,
        }
    }
}

impl<T> Mul<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, b: T) -> Self {
        Self {
            real: self.real * b.into(),
            img: self.img * b.into(),
        }
    }
}

impl MulAssign<&C64> for C64 {
    #[inline(always)]
    fn mul_assign(&mut self, b: &Self) {
        *self = Self {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + self.real * b.img,
        }
    }
}

impl MulAssign<C64> for C64 {
    #[inline(always)]
    fn mul_assign(&mut self, b: Self) {
        *self = Self {
            real: self.real * b.real - self.img * b.img,
            img: self.img * b.real + self.real * b.img,
        }
    }
}

impl<T> MulAssign<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    #[inline(always)]
    fn mul_assign(&mut self, b: T) {
        self.real *= b.into();
        self.img *= b.into();
    }
}

impl Div<&C64> for C64 {
    type Output = Self;
    #[inline]
    fn div(self, b: &Self) -> Self {
        let abs = b.abs_sq();
        Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl Div<C64> for C64 {
    type Output = Self;
    #[inline(always)]
    fn div(self, b: Self) -> Self {
        let abs = b.abs_sq();
        Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl<T> Div<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    type Output = Self;
    #[inline(always)]
    fn div(self, b: T) -> Self {
        Self {
            real: self.real / b.into(),
            img: self.img / b.into(),
        }
    }
}

impl DivAssign<C64> for C64 {
    #[inline(always)]
    fn div_assign(&mut self, b: Self) {
        let abs = b.abs_sq();
        *self = Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl DivAssign<&C64> for C64 {
    #[inline(always)]
    fn div_assign(&mut self, b: &Self) {
        let abs = b.abs_sq();
        *self = Self {
            real: (self.real * b.real + self.img * b.img) / abs,
            img: (self.img * b.real - self.real * b.img) / abs,
        }
    }
}

impl<T> DivAssign<T> for C64
where
    T: Number + core::convert::Into<f64> + Clone + Copy,
{
    #[inline(always)]
    fn div_assign(&mut self, b: T) {
        self.real /= b.into();
        self.img /= b.into();
    }
}

impl core::fmt::Display for C64 {
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
        impl core::convert::From<$type> for C64 {
            #[inline(always)]
            fn from(item: $type) -> C64 {
                C64 {
                    real: (item as f64),
                    img: 0.0,
                }
            }
        }

        impl core::convert::From<C64> for $type {
            #[inline(always)]
            fn from(item: C64) -> Self {
                item.real as $type
            }
        }

        impl Add<C64> for $type {
            type Output = C64;
            #[inline(always)]
            fn add(self, b: C64) -> C64 {
                C64 {
                    real: b.real + (self as f64),
                    img: b.img,
                }
            }
        }

        impl AddAssign<C64> for $type {
            #[inline(always)]
            fn add_assign(&mut self, b: C64) {
                *self += (b.real as $type);
            }
        }

        impl Sub<C64> for $type {
            type Output = C64;
            #[inline(always)]
            fn sub(self, b: C64) -> C64 {
                C64 {
                    real: -b.real + (self as f64),
                    img: -b.img,
                }
            }
        }

        impl SubAssign<C64> for $type {
            #[inline(always)]
            fn sub_assign(&mut self, b: C64) {
                *self -= (b.real as $type);
            }
        }

        impl Mul<C64> for $type {
            type Output = C64;
            #[inline(always)]
            fn mul(self, b: C64) -> C64 {
                C64 {
                    real: b.real * (self as f64),
                    img: b.img * (self as f64),
                }
            }
        }

        impl MulAssign<C64> for $type {
            #[inline(always)]
            fn mul_assign(&mut self, b: C64) {
                *self *= (b.real as $type);
            }
        }

        impl Div<C64> for $type {
            type Output = C64;
            #[inline(always)]
            fn div(self, b: C64) -> C64 {
                let slf = (self as f64);
                let abs = b.abs_sq();
                C64 {
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

impl Default for C64 {
    #[inline(always)]
    fn default() -> Self {
        Self::zero()
    }
}

impl C64 {
    #[inline(always)]
    pub fn new(r: f64, i: f64) -> Self {
        Self { real: r, img: i }
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self {
            real: 0.0,
            img: 0.0,
        }
    }

    #[inline(always)]
    pub fn unit() -> Self {
        Self {
            real: 1.0,
            img: 0.0,
        }
    }

    #[inline(always)]
    pub fn unit_ag(ag: f64) -> Self {
        Self {
            real: ag.cos(),
            img: ag.sin(),
        }
    }

    #[inline(always)]
    pub fn unit_ag_conj(ag: f64) -> Self {
        Self {
            real: ag.cos(),
            img: -ag.sin(),
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

    #[inline]
    pub fn pow(self, mut power: u32) -> Self {
        match power {
            0 => C64::unit(),
            1 => self,
            2 => self * self,
            3 => self * self * self,
            _ => {
                let (mut result, mut mul) = (Self::unit(), self);

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

    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self {
            real: self.real,
            img: -self.img,
        }
    }
}
