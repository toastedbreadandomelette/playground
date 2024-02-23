use super::fast_dft;
use crate::utils::index_generator::IndexGen;
use crate::utils::{
    c64::{Number, C64, PI},
    c64x2::C64x2,
};
use core::convert::Into;
use core::ops::{Add, AddAssign, Mul};
use vector::Vector;

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
unsafe fn join_2(input: &mut [C64], _: bool) {
    let scal = C64x2::from_array([C64::new(1.0, 1.0), C64::new(-1.0, -1.0)]);

    input.chunks_exact_mut(2).for_each(|chunk| {
        let (mut mul0, smul0) =
            (C64x2::splat(chunk[0]), C64x2::splat(chunk[1]));

        mul0 += smul0.scalar_mul_vec(scal);
        mul0.copy_to_slice(chunk);
    });
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
unsafe fn join_4(input: &mut [C64], is_inverse: bool) {
    let scal = C64x2::from_array([
        C64::unit(),
        if !is_inverse {
            C64::new(0.0, -1.0)
        } else {
            C64::new(0.0, 1.0)
        },
    ]);

    input.chunks_exact_mut(4).for_each(|chunk| {
        let (u1, v1) =
            (C64x2::from_slice(chunk), C64x2::from_slice(&chunk[2..]));
        let t1 = v1 * scal;

        (u1 + t1).copy_to_slice(chunk);
        (u1 - t1).copy_to_slice(&mut chunk[2..]);
    });
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
unsafe fn join_generic(input: &mut [C64], block_size: usize, is_inverse: bool) {
    let angle: f64 = 2.0 * PI / (block_size as f64);
    let winit = if !is_inverse {
        C64::unit_ag_conj(angle)
    } else {
        C64::unit_ag(angle)
    };

    let wmul = winit;
    let wmul2 = wmul * wmul;
    let wmul3 = wmul2 * wmul;
    let wmul4 = wmul3 * wmul;
    let wmul5 = wmul4 * wmul;
    let wmul6 = wmul5 * wmul;
    let wmul7 = wmul6 * wmul;
    let wmul8 = wmul7 * wmul;

    let wsmul2 = C64x2::splat(wmul2);
    let wsmul8 = C64x2::splat(wmul8);

    let half_block = block_size >> 1;

    input.chunks_exact_mut(block_size).for_each(|chunk| {
        let mut ws0 = C64x2::from_array([C64::unit(), winit]);
        let mut ws1 = ws0 * wsmul2;
        let mut ws2 = ws1 * wsmul2;
        let mut ws3 = ws2 * wsmul2;

        let (first_block, second_block) = chunk.split_at_mut(half_block);

        first_block
            .chunks_exact_mut(8)
            .zip(second_block.chunks_exact_mut(8))
            .for_each(|(first, second)| {
                let (u0, u1, u2, u3) = (
                    C64x2::from_slice(first),
                    C64x2::from_slice(&first[2..]),
                    C64x2::from_slice(&first[4..]),
                    C64x2::from_slice(&first[6..]),
                );
                let (v0, v1, v2, v3) = (
                    C64x2::from_slice(second),
                    C64x2::from_slice(&second[2..]),
                    C64x2::from_slice(&second[4..]),
                    C64x2::from_slice(&second[6..]),
                );

                let (t0, t1, t2, t3) = (v0 * ws0, v1 * ws1, v2 * ws2, v3 * ws3);

                (u0 + t0).copy_to_slice(first);
                (u1 + t1).copy_to_slice(&mut first[2..]);
                (u2 + t2).copy_to_slice(&mut first[4..]);
                (u3 + t3).copy_to_slice(&mut first[6..]);

                (u0 - t0).copy_to_slice(second);
                (u1 - t1).copy_to_slice(&mut second[2..]);
                (u2 - t2).copy_to_slice(&mut second[4..]);
                (u3 - t3).copy_to_slice(&mut second[6..]);

                ws0 *= wsmul8;
                ws1 *= wsmul8;
                ws2 *= wsmul8;
                ws3 *= wsmul8;
            });

        let mut v: [C64; 2] = [C64::zero(); 2];
        ws0.copy_to_slice(&mut v);
        let mut w = v[0];

        first_block
            .chunks_exact_mut(8)
            .into_remainder()
            .iter_mut()
            .zip(second_block.chunks_exact_mut(8).into_remainder().iter_mut())
            .for_each(|(first, second)| {
                let (u, v) = (*first, *second * w);
                (*first, *second) = (u + v, u - v);
                w *= winit;
            });
    });
}

/// Perform Fast Fourier Transform
/// on `n` values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method, and non-recursive method
pub fn faster_fft<T>(array: &[T]) -> Vector<C64>
where
    T: Number + AddAssign + Mul + Add + Into<f64> + Copy,
    f64: From<T>,
    C64: From<T>,
{
    let n = array.len();

    if (n & 1) == 1 || n < 16 {
        fast_dft::dft_fast(array)
    } else {
        let index_iter: IndexGen = IndexGen::new(array.len());
        let mut block_size = index_iter.get_base_size();

        let mut input: Vector<C64> = Vector::zeroed(array.len());
        input
            .iter_mut()
            .zip(IndexGen::new(array.len()).map(|x| array[x]))
            .for_each(|(x, element)| {
                *x = C64::from(element);
            });

        if block_size > 1 {
            input.chunks_exact_mut(block_size).for_each(|chunk| {
                let res = fast_dft::dft_fast_c64(chunk);
                chunk.copy_from_slice(&res);
            });
        }

        block_size <<= 1;

        while block_size <= n {
            match block_size {
                2 => unsafe { join_2(&mut input, false) },
                4 => unsafe { join_4(&mut input, false) },
                _ => unsafe { join_generic(&mut input, block_size, false) },
            }

            block_size <<= 1;
        }

        input
    }
}

/// Perform Fast Fourier Transform
/// on `n` values of Vector, and returns the floating values
///
/// Uses Divide-and-Conquer method, and non-recursive method
pub fn faster_ifft<T>(array: &[C64]) -> Vector<T>
where
    T: Number
        + AddAssign
        + Mul
        + Add
        + core::convert::Into<f64>
        + core::convert::From<f64>
        + Copy,
    f64: From<T>,
    C64: From<T>,
{
    let n = array.len();
    if (n & 1) == 1 || n < 16 {
        fast_dft::idft_fast::<T>(array)
    } else {
        let index_iter: IndexGen = IndexGen::new(array.len());
        let mut block_size = index_iter.get_base_size();

        let mut input: Vector<C64> = Vector::zeroed(array.len());
        input
            .iter_mut()
            .zip(IndexGen::new(array.len()).map(|x| array[x]))
            .for_each(|(x, element)| {
                *x = element;
            });

        if block_size > 1 {
            input.chunks_exact_mut(block_size).for_each(|chunk| {
                let res = &fast_dft::idft_fast_c64(chunk);
                chunk.copy_from_slice(res);
            });
        }

        block_size <<= 1;

        while block_size <= n {
            match block_size {
                2 => unsafe { join_2(&mut input, true) },
                4 => unsafe { join_4(&mut input, true) },
                _ => unsafe { join_generic(&mut input, block_size, true) },
            }

            block_size <<= 1;
        }

        let len = input.len();
        let len_inv = 1.0 / (len as f64);
        let mut res: Vector<T> = Vector::zeroed(len);
        res.iter_mut()
            .zip(input.iter())
            .for_each(|(res, inp)| *res = (inp.real * len_inv).into());

        res
    }
}
