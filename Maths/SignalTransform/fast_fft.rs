// mod discrete_fourier_transform;
mod complex;
use crate::complex::{Complex, Number, PI};
use std::ops::{Add, AddAssign, Mul};

pub fn fast_fft<T>(array: &Vec<T>) -> Vec<Complex>
where
    T: Number + AddAssign + Mul + Add + std::convert::Into<f64> + Copy,
    f64: From<T>,
{
    let dft = |array: &mut Vec<Complex>, start: usize, end: usize| {
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); end - start];
        let size = end - start;
        let angle = 2.0 * PI / (size as f64);
        let wlen = Complex::new(angle.cos(), -angle.sin());
        let mut wstart = wlen;

        for index in start..end {
            result[0] += array[index];
        }

        for index in 1..size {
            let mut w = Complex::new(1.0, 0.0);
            for i in start..end {
                result[index] += array[i] * w;
                w *= wstart;
            }
            wstart *= wlen;
        }

        for x in start..end {
            array[x] = result[x - start];
        }
    };
    let n = array.len();
    if (n & 1) == 1 || n < 16 {
        let mut input: Vec<Complex> = array
            .into_iter()
            .map(|x| Complex::new(f64::from(*x), 0.0))
            .collect::<Vec<Complex>>();
        dft(&mut input, 0, n);
        input
    } else {
        // vec![Complex::new(1.0, 0.0); 10]
        let ls = ((n ^ (n - 1)) + 1) >> 1;
        let mut indexes: Vec<usize> = vec![0; n];
        let (mut j, mut i) = (1, n);
        while (i & 1) == 0 {
            for k in (i >> 1)..i {
                indexes[k] = j;
            }
            j <<= 1;
            i >>= 1;
        }

        for k in 1..i {
            indexes[k] = indexes[k - 1] + ls;
            indexes[k + (n >> 1)] = indexes[k] + 1;
        }
        let mut index = i;
        while index < (n >> 1) {
            for k in 0..index {
                indexes[k + index] += indexes[k];
                indexes[k + index + (n >> 1)] = indexes[k + index] + 1;
            }
            index <<= 1;
        }

        let mut input: Vec<Complex> = (0..n)
            .into_iter()
            .map(|x| Complex::new(f64::from(array[indexes[x]]), 0.0))
            .collect::<Vec<Complex>>();

        if i > 1 {
            // while i < n && i < 8 {
            //     i <<= 1;
            // }
            for index in (0..n).step_by(i) {
                dft(&mut input, index, index + i);
            }
        }

        let mut block_size = i << 1;
        while block_size <= n {
            let angle = 2.0 * PI / (block_size as f64);
            let winit = Complex::new(angle.cos(), -angle.sin());
            for i in (0..n).step_by(block_size) {
                let mut w = Complex::new(1.0, 0.0);
                for j in 0..(block_size >> 1) {
                    let (u, v) = (input[i + j], input[i + j + (block_size >> 1)] * w);
                    input[i + j] = u + v;
                    input[i + j + (block_size >> 1)] = u - v;
                    w *= winit;
                }
            }
            block_size <<= 1;
        }

        input
    }
}

pub fn fast_ifft<T>(array: &Vec<Complex>) -> Vec<T>
where
    T: Number + AddAssign + Mul + Add + std::convert::From<f64> + Copy,
    f64: From<T>,
{
    let idft = |array: &mut Vec<Complex>, start: usize, end: usize| {
        let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); end - start];
        let size = end - start;
        let angle = 2.0 * PI / (size as f64);
        let wlen = Complex::new(angle.cos(), angle.sin());
        let mut wstart = wlen;

        for index in start..end {
            result[0] += array[index];
        }

        for index in 1..size {
            let mut w = Complex::new(1.0, 0.0);
            for i in start..end {
                result[index] += array[i] * w;
                w *= wstart;
            }
            wstart *= wlen;
        }

        for x in start..end {
            array[x] = result[x - start];
        }
    };
    let n = array.len();
    if n & 1 == 1 {
        let mut input: Vec<Complex> = array.into_iter().map(|x| *x).collect::<Vec<Complex>>();
        idft(&mut input, 0, n);
        input
            .into_iter()
            .map(|val| T::from(val.real / (n as f64)))
            .collect::<Vec<T>>()
    } else {
        let ls = ((n ^ (n - 1)) + 1) >> 1;
        let mut indexes: Vec<usize> = vec![0; n];
        let (mut j, mut i) = (1, n);
        while (i & 1) == 0 {
            for k in (i >> 1)..i {
                indexes[k] = j;
            }
            j <<= 1;
            i >>= 1;
        }

        for k in 1..i {
            indexes[k] = indexes[k - 1] + ls;
            indexes[k + (n >> 1)] = indexes[k] + 1;
        }
        let mut index = i;
        while index < (n >> 1) {
            for k in 0..index {
                indexes[k + index] += indexes[k];
                indexes[k + index + (n >> 1)] = indexes[k + index] + 1;
            }
            index <<= 1;
        }

        let mut input: Vec<Complex> = (0..n)
            .into_iter()
            .map(|x| array[indexes[x]])
            .collect::<Vec<Complex>>();

        if i > 1 {
            for index in (0..n).step_by(i) {
                idft(&mut input, index, index + i);
            }
        }

        let mut block_size = i << 1;
        while block_size <= n {
            let angle = 2.0 * PI / (block_size as f64);
            let winit = Complex::new(angle.cos(), angle.sin());
            for i in (0..n).step_by(block_size) {
                let mut w = Complex::new(1.0, 0.0);
                for j in 0..(block_size >> 1) {
                    let (u, v) = (input[i + j], input[i + j + (block_size >> 1)] * w);
                    input[i + j] = u + v;
                    input[i + j + (block_size >> 1)] = u - v;
                    w *= winit;
                }
            }
            block_size <<= 1;
        }

        input
            .into_iter()
            .map(|val| T::from(val.real / (n as f64)))
            .collect::<Vec<T>>()
    }
}

#[test]
pub fn test_fft_ifft() {
    let sz = 1048576;
    let inp = (0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    let orig: Vec<f64> = fast_ifft(&val);

    assert!(orig
        .iter()
        .enumerate()
        .all(|(index, elem)| *elem - inp[index] < 1E-5_f64));
}

#[test]
pub fn test_fft_ifft_small() {
    for sz in vec![8, 16, 24, 32, 40, 48, 56, 64, 72, 80] {
        let inp = (0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
        let val = fast_fft::<f64>(&inp);
        let orig: Vec<f64> = fast_ifft(&val);

        assert!(orig
            .iter()
            .enumerate()
            .all(|(index, elem)| *elem - inp[index] < 1E-5_f64));
    }
}

#[test]
pub fn test_fft_ifft_without_2_power() {
    let sz = 524288 + 262144;
    let inp = (0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    let orig: Vec<f64> = fast_ifft(&val);

    assert!(orig
        .iter()
        .enumerate()
        .all(|(index, elem)| *elem - inp[index] < 1E-5_f64));
}

fn main() {
    let sz = 24;
    let inp = (0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    println!("{:?}", val);
    let orig: Vec<f64> = fast_ifft(&val);

    println!("{:?}", orig);
}
