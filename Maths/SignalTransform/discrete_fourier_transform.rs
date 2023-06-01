mod complex;
use crate::complex::{Complex, Number, PI};
use std::ops::{AddAssign, Mul};

pub fn dft<T: From<T> + AddAssign + Mul + Copy + Number>(arr: &Vec<T>) -> Vec<Complex>
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

pub fn idft<T: From<T> + AddAssign + Mul + Copy + Number + std::convert::From<f64>>(
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

pub fn fft<T: From<T> + AddAssign + Mul + Copy + Number + std::default::Default>(
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

#[test]
pub fn test_fft_and_ifft() {
    let sz = 1048576;
    let inp = (0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    let val = fft::<f64>(&inp);
    let orig: Vec<f64> = ifft(&val);

    assert!(orig
        .iter()
        .enumerate()
        .all(|(index, elem)| *elem - inp[index] < 1E-5_f64));
}
