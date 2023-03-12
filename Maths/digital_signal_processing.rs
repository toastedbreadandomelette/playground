pub mod complex {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    pub const PI: f64 = 3.141592653589793238462643383279;

    // Complex number for computing.
    #[derive(Debug, Copy, Clone)]
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
        T: Number + std::convert::Into<f64> + Clone + Copy,
    {
        fn sub_assign(&mut self, b: T) {
            self.real -= b.into();
        }
    }

    impl<T> Sub<T> for Complex
    where
        T: Number + std::convert::Into<f64> + Clone + Copy,
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
        T: Number + std::convert::Into<f64> + Clone + Copy,
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
        T: Number + std::convert::Into<f64> + Clone + Copy,
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
        T: Number + std::convert::Into<f64> + Clone + Copy,
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
        T: Number + std::convert::Into<f64> + Clone + Copy,
    {
        fn mul_assign(&mut self, b: T) {
            self.real *= b.into();
            self.img *= b.into();
        }
    }

    impl Div<&Complex> for Complex {
        type Output = Self;
        fn div(self, b: &Complex) -> Self {
            let abs = b.real * b.real + b.img * b.img;
            Self {
                real: (self.real * b.real + self.img * b.img) / abs,
                img: (self.img * b.real - self.real * b.img) / abs,
            }
        }
    }

    impl Div<Complex> for Complex {
        type Output = Self;
        fn div(self, b: Complex) -> Self {
            let abs = b.real * b.real + b.img * b.img;
            Self {
                real: (self.real * b.real + self.img * b.img) / abs,
                img: (self.img * b.real - self.real * b.img) / abs,
            }
        }
    }

    impl<T> Div<T> for Complex
    where
        T: Number + std::convert::Into<f64> + Clone + Copy,
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
            let abs = b.real * b.real + b.img * b.img;
            *self = Self {
                real: (self.real * b.real + self.img * b.img) / abs,
                img: (self.img * b.real - self.real * b.img) / abs,
            }
        }
    }

    impl DivAssign<&Complex> for Complex {
        fn div_assign(&mut self, b: &Self) {
            let abs = b.real * b.real + b.img * b.img;
            *self = Self {
                real: (self.real * b.real + self.img * b.img) / abs,
                img: (self.img * b.real - self.real * b.img) / abs,
            }
        }
    }

    impl<T> DivAssign<T> for Complex
    where
        T: Number + std::convert::Into<f64> + Clone + Copy,
    {
        fn div_assign(&mut self, b: T) {
            self.real /= b.into();
            self.img /= b.into();
        }
    }

    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.real == 0.0 && self.img == 0.0 {
                f.write_str("0")
            } else if self.real == 0.0 {
                f.write_str(format!("{}i", self.img).as_str())
            } else if self.img == 0.0 {
                f.write_str(format!("{}", self.real).as_str())
            } else {
                if self.img > 0.0 {
                    f.write_str(format!("{}+{}i", self.real, self.img).as_str())
                } else {
                    f.write_str(format!("{}{}i", self.real, self.img).as_str())
                }
            }
        }
    }

    macro_rules! impl_op_for_type {
        ($type: ident) => {
            impl std::convert::From<$type> for Complex {
                fn from(item: $type) -> Complex {
                    Complex {
                        real: (item as f64),
                        img: 0.0,
                    }
                }
            }

            impl std::convert::From<Complex> for $type {
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
                    let abs = b.abs();
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
        pub fn new(r: f64, i: f64) -> Self {
            Self { real: r, img: i }
        }

        pub fn abs_sq(self) -> f64 {
            self.real * self.real + self.img * self.img
        }

        pub fn abs(self) -> f64 {
            (self.real * self.real + self.img * self.img).sqrt()
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
}

use std::ops::{AddAssign, Mul};

use complex::{Complex, PI};

pub fn dft<T: From<T> + AddAssign + Mul + Copy + complex::Number>(arr: &Vec<T>) -> Vec<Complex>
where
    f64: From<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), -angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
}

pub fn idft<T: From<T> + AddAssign + Mul + Copy + complex::Number + std::convert::From<f64>>(
    arr: &Vec<Complex>,
) -> Vec<T>
where
    f64: Into<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
        .iter()
        .map(|x| (x.real / result.len() as f64).into())
        .collect::<Vec<T>>()
}

pub fn idft_complex(arr: &Vec<Complex>) -> Vec<Complex> {
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
}

pub fn fft<T: From<T> + AddAssign + Mul + Copy + complex::Number + std::default::Default>(
    arr: &Vec<T>,
) -> Vec<Complex>
where
    f64: From<T>,
{
    if arr.len() < 16 || arr.len() & 1 == 1 {
        dft(arr)
    } else {
        let even = (0..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<T>>();
        let odd = (1..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<T>>();

        let odd_fft = fft(&odd);
        let even_fft = fft(&even);

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), -angle.sin());
        let mut w = Complex::new(1.0, 0.0);
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
        for x in 0..(arr.len() / 2) {
            result[x] = even_fft[x] + odd_fft[x] * w;
            result[x + arr.len() / 2] = even_fft[x] - odd_fft[x] * w;
            w *= wlen;
        }
        result
    }
}

pub fn ifft_internal(arr: &Vec<Complex>) -> Vec<Complex> {
    if arr.len() < 16 || arr.len() & 1 == 1 {
        idft_complex(arr)
    } else {
        let even = (0..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<Complex>>();
        let odd = (1..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<Complex>>();

        let odd_fft = ifft_internal(&odd);
        let even_fft = ifft_internal(&even);

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), angle.sin());
        let mut w = Complex::new(1.0, 0.0);
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
        for x in 0..(arr.len() / 2) {
            result[x] = even_fft[x] + odd_fft[x] * w;
            result[x + arr.len() / 2] = even_fft[x] - odd_fft[x] * w;
            w *= wlen;
        }
        result
    }
}

pub fn ifft(arr: &Vec<Complex>) -> Vec<f64> {
    ifft_internal(&arr)
        .iter()
        .map(|x| x.real / (arr.len() as f64))
        .collect::<Vec<f64>>()
}

pub fn main() {
    let sz = 1048576;
    let val = fft::<f64>(&(0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>());
    // for x in &val {
    //     print!("({:.4}), ", x);
    // }
    println!("");
    let orig: Vec<f64> = ifft(&val);
    // for x in orig {
    //     print!("({:.1}), ", x);
    // }
}
