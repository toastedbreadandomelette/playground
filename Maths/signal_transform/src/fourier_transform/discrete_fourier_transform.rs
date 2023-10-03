use crate::utils::complex::{Complex, Number, PI};
use core::ops::{AddAssign, Mul};

/// Perform Discrete Fourier Transform on n values of Vector, and returns the complex
///
/// Can be retrieved by performing idft
pub fn dft<T: From<T> + AddAssign + Mul + Copy + Number>(
    arr: &Vec<T>,
) -> Vec<Complex>
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

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = Complex::new(1.0, 0.0);
        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart;
        });
        wstart *= wlen;
    });

    result
}

/// Perform Inverse Discrete Fourier Transform on n values of Vector, and returns the floating values
pub fn idft<
    T: From<T> + AddAssign + Mul + Copy + Number + core::convert::From<f64>,
>(
    arr: &Vec<Complex>,
) -> Vec<T>
where
    f64: Into<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];

    result[0] = arr.iter().fold(Complex::zero(), |prev, curr| prev + *curr);

    let len = result.len() as f64;

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = Complex::new(1.0, 0.0);
        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart;
        });
        wstart *= wlen;

        *res /= len;
    });

    result.iter().map(|x| x.real.into()).collect::<Vec<T>>()
}

/// Perform Discrete Fourier Transform on n values of Vector, and returns the complex values
pub fn idft_complex(arr: &Vec<Complex>) -> Vec<Complex> {
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::zero(); arr.len()];
    result[0] = arr.iter().fold(Complex::zero(), |prev, curr| prev + *curr);

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = Complex::new(1.0, 0.0);
        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart;
        });
        wstart *= wlen;
    });

    result
}

/// Perform Fast Fourier Transform on n values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method.
pub fn fft<
    T: From<T> + AddAssign + Mul + Copy + Number + core::default::Default,
>(
    arr: &Vec<T>,
) -> Vec<Complex>
where
    f64: From<T>,
{
    if arr.len() < 8 || arr.len() & 1 == 1 {
        dft(arr)
    } else {
        let (odd_fft, even_fft) = {
            let even = arr.iter().step_by(2).copied().collect();
            let odd = arr.iter().skip(1).step_by(2).copied().collect();
            (fft(&odd), fft(&even))
        };

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), -angle.sin());
        let mut w = Complex::new(1.0, 0.0);
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
        odd_fft.iter().zip(even_fft.iter()).enumerate().for_each(
            |(x, (odd, even))| {
                let t = *odd * w;
                result[x] = *even + t;
                result[x + arr.len() / 2] = *even - t;
                w *= wlen;
            },
        );
        result
    }
}

/// Perform Inverse Fast Fourier Transform (only for internal purposes)
/// on n values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method.
pub fn ifft_internal(arr: &Vec<Complex>) -> Vec<Complex> {
    if arr.len() < 8 || arr.len() & 1 == 1 {
        idft_complex(arr)
    } else {
        let (odd_fft, even_fft) = {
            let even = arr.iter().step_by(2).copied().collect();
            let odd = arr.iter().skip(1).step_by(2).copied().collect();

            (ifft_internal(&odd), ifft_internal(&even))
        };

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), angle.sin());
        let mut w = Complex::new(1.0, 0.0);
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
        odd_fft.iter().zip(even_fft.iter()).enumerate().for_each(
            |(x, (odd, even))| {
                let t = *odd * w;
                result[x] = *even + t;
                result[x + arr.len() / 2] = *even - t;
                w *= wlen;
            },
        );

        result
    }
}

/// Perform Fast Fourier Transform on n values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method.
pub fn ifft(arr: &Vec<Complex>) -> Vec<f64> {
    let len = arr.len() as f64;
    ifft_internal(arr)
        .iter()
        .map(|x| x.real / len)
        .collect::<Vec<f64>>()
}

#[test]
pub fn test_fft_and_ifft() {
    let sz = 1048576;
    let inp = (0..sz).map(|x| x as f64).collect();
    let val = fft::<f64>(&inp);
    let orig: Vec<f64> = ifft(&val);

    assert!(orig
        .iter()
        .zip(inp)
        .all(|(elem, inp)| (*elem - inp).abs() < 1e-4));
}
