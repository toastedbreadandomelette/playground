use crate::utils::c64::{Number, C64, PI};
use core::ops::{AddAssign, Mul};

/// Perform Discrete Fourier Transform on n values of Vector, and returns the C64
///
/// Can be retrieved by performing idft
pub fn dft<T: From<T> + AddAssign + Mul + Copy + Number>(arr: &[T]) -> Vec<C64>
where
    f64: From<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = C64::new(angle.cos(), -angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<C64> = vec![C64::zero(); arr.len()];

    result[0] = arr.iter().fold(C64::zero(), |prev, curr| prev + *curr);

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = C64::unit();
        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart;
        });
        wstart *= wlen;
    });

    result
}

/// Perform Inverse Discrete Fourier Transform on n values of
/// Vector, and returns the floating values
pub fn idft<T>(arr: &[C64]) -> Vec<T>
where
    T: From<T> + AddAssign + Mul + Copy + Number + core::convert::From<f64>,
    f64: Into<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = C64::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<C64> = vec![C64::zero(); arr.len()];

    result[0] = arr.iter().fold(C64::zero(), |prev, curr| prev + *curr);

    let len = result.len() as f64;

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = C64::unit();
        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart;
        });
        wstart *= wlen;
    });
    let len = arr.len();

    result
        .iter()
        .map(|x| (*x / (len as f64)).real.into())
        .collect::<Vec<T>>()
}

/// Perform Discrete Fourier Transform on n values of Vector, and returns the C64 values
pub fn idft_c64(arr: &Vec<C64>) -> Vec<C64> {
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = C64::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<C64> = vec![C64::zero(); arr.len()];
    result[0] = arr.iter().fold(C64::zero(), |prev, curr| prev + *curr);

    result.iter_mut().skip(1).for_each(|res| {
        let mut w = C64::unit();
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
pub fn fft<T>(arr: &[T]) -> Vec<C64>
where
    T: From<T> + AddAssign + Mul + Copy + Number + core::default::Default,
    f64: From<T>,
{
    if arr.len() < 8 || arr.len() & 1 == 1 {
        dft(arr)
    } else {
        let (odd_fft, even_fft) = {
            let even: Vec<T> = arr.iter().step_by(2).copied().collect();
            let odd: Vec<T> = arr.iter().skip(1).step_by(2).copied().collect();
            (fft(&odd), fft(&even))
        };

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = C64::new(angle.cos(), -angle.sin());

        let mut w = C64::unit();
        let mut result: Vec<C64> = vec![C64::zero(); arr.len()];

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
pub fn ifft_internal(arr: &Vec<C64>) -> Vec<C64> {
    if arr.len() < 8 || arr.len() & 1 == 1 {
        idft_c64(arr)
    } else {
        let (odd_fft, even_fft) = {
            let even = arr.iter().step_by(2).copied().collect();
            let odd = arr.iter().skip(1).step_by(2).copied().collect();

            (ifft_internal(&odd), ifft_internal(&even))
        };

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = C64::unit_ag(angle);

        let mut w = C64::unit();
        let mut result: Vec<C64> = vec![C64::zero(); arr.len()];

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
pub fn ifft(arr: &Vec<C64>) -> Vec<f64> {
    let len = arr.len() as f64;
    ifft_internal(arr)
        .iter()
        .map(|x| x.real / len)
        .collect::<Vec<f64>>()
}

#[test]
pub fn test_dft_idft() {
    let sz = 8192;
    let inp: Vec<f64> = (0..sz).map(|x| x as f64).collect();
    let val = dft::<f64>(&inp);
    let orig: Vec<f64> = idft::<f64>(&val);

    assert!(orig
        .iter()
        .zip(inp.iter())
        .all(|(elem, inp)| { (*elem - inp).abs() < 1e-4 + 1e-4 * elem.abs() }));
}

#[test]
pub fn test_rec_fft_and_ifft() {
    let sz = 1048576;
    let inp: Vec<f64> = (0..sz).map(|x| x as f64).collect();
    let val = fft::<f64>(&inp);
    let orig: Vec<f64> = ifft(&val);

    assert!(orig
        .iter()
        .zip(inp)
        .all(|(elem, inp)| (*elem - inp).abs() < 1e-4 + 1e-4 * elem.abs()));
}
