use crate::utils::c64::{Number, C64, PI};
use core::ops::{Add, AddAssign, Mul};
use vector::Vector;

/// Perform Fast Fourier Transform
/// on `n` values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method, and non-recursive method
pub fn fast_fft<T>(array: &[T]) -> Vector<C64>
where
    T: Number + AddAssign + Mul + Add + core::convert::Into<f64> + Copy,
    f64: From<T>,
{
    let dft = |array: &mut [C64]| {
        let mut result = Vector::<C64>::zeroed(array.len());
        let size = array.len();
        let angle = 2.0 * PI / (size as f64);
        let wlen = C64::unit_ag_conj(angle);
        let mut wstart = wlen;

        result[0] = array.iter().fold(C64::zero(), |prev, curr| prev + *curr);

        result.iter_mut().skip(1).for_each(|elem| {
            let mut w = C64::unit();
            array.iter().for_each(|val| {
                *elem += *val * w;
                w *= wstart;
            });
            wstart *= wlen;
        });

        array.copy_from_slice(&result);
    };

    let n = array.len();
    if (n & 1) == 1 {
        let mut input: Vector<C64> =
            array.iter().map(|x| C64::new(f64::from(*x), 0.0)).collect();
        dft(&mut input);
        input
    } else {
        let ls = ((n ^ (n - 1)) + 1) >> 1;
        let mut indexes: Vector<usize> = Vector::zeroed(n);
        let (mut j, mut i) = (1, n);
        // This shuffling method is done for general FFT method.
        // If MSB is smaller, this method works faster of the order
        // n log(n), otherwise, runs at O(n2).
        while (i & 1) == 0 {
            indexes[i >> 1..i].fill(j);
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

        let mut input: Vector<C64> = indexes
            .iter()
            .map(|x| C64::new(f64::from(array[*x]), 0.0))
            .collect();
        // display_bin(&indexes);

        if i > 1 {
            for index in (0..n).step_by(i) {
                dft(&mut input[index..index + i]);
            }
        }

        let mut block_size = i << 1;
        while block_size <= n {
            let angle = 2.0 * PI / (block_size as f64);
            let winit = C64::unit_ag_conj(angle);
            for i in (0..n).step_by(block_size) {
                let mut w = C64::unit();
                for j in 0..(block_size >> 1) {
                    let (u, v) =
                        (input[i + j], input[i + j + (block_size >> 1)] * w);
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

/// Perform Inverse Fast Fourier Transform
/// on n values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method, and non-recursive method
pub fn fast_ifft<T>(array: &[C64]) -> Vec<T>
where
    T: Number + AddAssign + Mul + Add + core::convert::From<f64> + Copy,
    f64: From<T>,
    C64: From<T>,
{
    let idft = |array: &mut [C64]| {
        let mut result: Vec<C64> = vec![C64::new(0.0, 0.0); array.len()];
        let size = array.len();
        let angle = 2.0 * PI / (size as f64);
        let wlen = C64::unit_ag(angle);
        let mut wstart = wlen;

        result[0] = array.iter().fold(C64::zero(), |prev, curr| prev + *curr);

        result.iter_mut().skip(1).for_each(|elem| {
            let mut w = C64::unit();
            array.iter().for_each(|val| {
                *elem += *val * w;
                w *= wstart;
            });
            wstart *= wlen;
        });

        array.copy_from_slice(&result);
    };
    let n = array.len();
    if n & 1 == 1 {
        let mut input: Vec<C64> = array.to_vec();
        idft(&mut input[..]);
        input
            .iter()
            .map(|val| T::from(val.real / (n as f64)))
            .collect::<Vec<T>>()
    } else {
        let ls = ((n ^ (n - 1)) + 1) >> 1;
        let mut indexes: Vec<usize> = vec![0; n];
        let (mut j, mut i) = (1, n);
        // This shuffling method is done for general FFT method.
        // If MSB is smaller, this method works faster of the order
        // n logn, otherwise, runs at O(n2).
        while (i & 1) == 0 {
            indexes[i >> 1..i].fill(j);
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

        let mut input: Vec<C64> =
            indexes.iter().map(|x| array[*x]).collect::<Vec<C64>>();

        if i > 1 {
            for l in (0..input.len()).step_by(i) {
                idft(&mut input[l..l + i]);
            }
        }

        let mut block_size = i << 1;
        while block_size <= n {
            let angle = 2.0 * PI / (block_size as f64);
            let winit = C64::unit_ag(angle);
            for i in (0..n).step_by(block_size) {
                let mut w = C64::unit();
                for j in 0..(block_size >> 1) {
                    let (u, v) =
                        (input[i + j], input[i + j + (block_size >> 1)] * w);
                    input[i + j] = u + v;
                    input[i + j + (block_size >> 1)] = u - v;
                    w *= winit;
                }
            }
            block_size <<= 1;
        }

        input
            .iter()
            .map(|val| T::from(val.real / (n as f64)))
            .collect::<Vec<T>>()
    }
}

#[test]
pub fn test_fft_ifft() {
    let sz = 1048576;
    let inp = (0..sz).map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    let orig: Vec<f64> = fast_ifft(&val);

    assert!(orig
        .iter()
        .zip(inp)
        .all(|(elem, inp)| (elem - inp).abs() < 1E-4_f64));
}

#[test]
pub fn test_fft_ifft_small() {
    for sz in vec![8, 16, 24, 32, 40, 48, 56, 64, 72, 80] {
        let inp = (0..sz).map(|x| x as f64).collect::<Vec<f64>>();
        let val = fast_fft::<f64>(&inp);
        let orig: Vec<f64> = fast_ifft(&val);

        assert!(orig
            .iter()
            .zip(inp)
            .all(|(elem, inp)| (*elem - inp).abs() < 1E-5_f64));
    }
}

#[test]
pub fn test_fft_ifft_without_2_power() {
    let sz = 524288 + 262144 + 131072 + 65536;
    let inp = (0..sz).map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    let orig: Vec<f64> = fast_ifft(&val);

    assert!(orig
        .iter()
        .zip(inp)
        .all(|(elem, inp)| (*elem - inp).abs() < 1E-4_f64));
}

#[test]
pub fn test_fft_ifft_with_2_power() {
    let sz = 1048576;
    let inp = (0..sz).map(|x| x as f64).collect::<Vec<f64>>();
    let val = fast_fft::<f64>(&inp);
    let orig: Vec<f64> = fast_ifft(&val);

    assert!(orig
        .iter()
        .zip(inp)
        .all(|(elem, inp)| (*elem - inp).abs() < 1E-4_f64));
}
