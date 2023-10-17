use crate::utils::c64::{Number, C64, PI};
use core::ops::{Add, AddAssign, Mul};

use super::faster_dft;

/// Perform Fast Fourier Transform
/// on `n` values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method, and non-recursive method
pub fn faster_fft<T>(array: &[T]) -> Vec<C64>
where
    T: Number + AddAssign + Mul + Add + core::convert::Into<f64> + Copy,
    f64: From<T>,
{
    let n = array.len();
    if (n & 1) == 1 || n < 16 {
        faster_dft::dft_fast(array)
    } else {
        // vec![C64::unit(); 10]
        let ls = ((n ^ (n - 1)) + 1) >> 1;
        let mut indexes: Vec<usize> = vec![0; n];
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

        let mut input: Vec<C64> = vec![C64::zero(); n];

        if i > 1 {
            for index in (0..n).step_by(i) {
                input[index..index + i].copy_from_slice(&faster_dft::dft_fast(
                    &indexes
                        .iter()
                        .skip(index)
                        .take(i)
                        .map(|c| array[*c])
                        .collect::<Vec<T>>(),
                ));
            }
        }

        let mut block_size = i << 1;
        while block_size <= n {
            let angle = 2.0 * PI / (block_size as f64);
            let winit = C64::unit_ag_conj(angle);
            let half_block = block_size >> 1;
            for i in (0..n).step_by(block_size) {
                let mut w = C64::unit();

                for j in 0..half_block {
                    let (u, v) = (input[i + j], input[i + j + half_block] * w);
                    
                    input[i + j] = u + v;
                    input[i + j + half_block] = u - v;
                    w *= winit;
                }
            }
            block_size <<= 1;
        }

        input
    }
}
