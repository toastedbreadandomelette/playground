use crate::common::transpose_vec;
use vector::vector::Vector;
use crate::matmul::cf_process_blocks::{
    iter_blocks_on_1xN, iter_blocks_on_2xN, iter_blocks_on_3xN,
    iter_blocks_on_NxN,
};
use core::simd::f64x4;
use core::simd::SimdFloat;

const BLOCKSIZE: usize = 32;

const R_BLOCKSIZE: usize = 32;
const C_BLOCKSIZE: usize = 32;

/// Cache friendly and blocked matrix multiplication of two matrices
/// `a` and `b` of shape `ashape (m x n)` and `bshape (n x p)` respectively
///
/// Uses SIMD functions: calls unsafe functions internally for computation
#[inline(always)]
pub fn cf_blocked_simd(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    unsafe { cf_blocked_simd_unsafe(a, b, ashape, bshape) }
}

/// We're computing values of 4x4 sub-matrix of [`c`].
/// We'll be computing 16 dot products at a time.
/// i.e. for submatrices,
///
/// Iterate on 4 of these rows of a_block at a time
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn cf_blocked_simd_unsafe(
    a: &[f64],
    b: &[f64],
    (m, n): (usize, usize),
    (_, p): (usize, usize),
) -> Vector<f64> {
    let mut c = Vector::zeroed(m * p);
    // Transposed matrix `b`, we're aware of
    // resultant shape
    let (tb, _) = transpose_vec(b, (n, p));
    let r_block_size = R_BLOCKSIZE;
    let c_block_size = C_BLOCKSIZE;

    a.chunks(n * r_block_size)
        .enumerate()
        .for_each(|(i_index, a_block)| {
            // ibl be starting point of block of rows for matrix `a`
            let ibl = i_index * r_block_size;
            tb.chunks(n * c_block_size).enumerate().for_each(
                |(j_index, b_block)| {
                    // jbl be starting point of block of rows for matrix `b`
                    let jbl = j_index * c_block_size;
                    // We're computing values of 4x4 sub-matrix of `c`.
                    // We'll be computing 16 dot products at a time.
                    // i.e. for submatrices,
                    //
                    // c_{i+0}{j}   c_{i+0}_{j+1}   c_{i+0}_{j+2}   c_{i+0}_{j+3}
                    // c_{i+1}{j}   c_{i+1}_{j+1}   c_{i+1}_{j+2}   c_{i+1}_{j+3}
                    // c_{i+2}{j}   c_{i+2}_{j+1}   c_{i+2}_{j+2}   c_{i+2}_{j+3}
                    // c_{i+3}{j}   c_{i+3}_{j+1}   c_{i+3}_{j+2}   c_{i+3}_{j+3}
                    //
                    // Iterate on 4 of these rows of a_block at a time
                    iter_blocks_on_NxN(
                        a_block, b_block, &mut c, ibl, jbl, n, p,
                    );

                    let a_block_len = a_block.len() / n;
                    let a_rem = a_block.chunks_exact(n * 4).remainder();

                    match a_rem.len() / n {
                        1 => iter_blocks_on_1xN(
                            a_rem,
                            b_block,
                            &mut c,
                            ibl + a_block_len - 1,
                            jbl,
                            n,
                            p,
                        ),
                        2 => iter_blocks_on_2xN(
                            a_rem,
                            b_block,
                            &mut c,
                            ibl + a_block_len - 2,
                            jbl,
                            n,
                            p,
                        ),
                        3 => iter_blocks_on_3xN(
                            a_rem,
                            b_block,
                            &mut c,
                            ibl + a_block_len - 3,
                            jbl,
                            n,
                            p,
                        ),
                        _ => {} // No cases to be considered
                    }
                },
            );
        });

    c
}

/// Same same but different
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn cf_block_transposed_multi_accumulated_simd_matmul_4x4(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vector<f64> = Vector::zeroed(m * p);
    let block_size = BLOCKSIZE;

    // Before computing matrix multiplication:
    // we transpose the matrix
    let (tb, _) = transpose_vec(b, (n, p));

    let irem = m % block_size;
    let jrem = p % block_size;

    let (mut ans00, mut ans01, mut ans02, mut ans03);
    let (mut ans10, mut ans11, mut ans12, mut ans13);
    let (mut ans20, mut ans21, mut ans22, mut ans23);
    let (mut ans30, mut ans31, mut ans32, mut ans33);

    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = if ibl + block_size > m - irem {
            m - irem
        } else {
            ibl + block_size
        };
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = if jbl + block_size > p - jrem {
                p - jrem
            } else {
                jbl + block_size
            };
            for i in (ibl..ilim).step_by(4) {
                for j in (jbl..jlim).step_by(4) {
                    (ans00, ans01, ans02, ans03) = (
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                    );
                    (ans10, ans11, ans12, ans13) = (
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                    );
                    (ans20, ans21, ans22, ans23) = (
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                    );
                    (ans30, ans31, ans32, ans33) = (
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                        f64x4::splat(0.0),
                    );
                    a[i * n..(i + 1) * n]
                        .windows(4)
                        .step_by(4)
                        .zip(a[(i + 1) * n..(i + 2) * n].windows(4).step_by(4))
                        .zip(a[(i + 2) * n..(i + 3) * n].windows(4).step_by(4))
                        .zip(a[(i + 3) * n..(i + 4) * n].windows(4).step_by(4))
                        .enumerate()
                        .for_each(|(idx, (((p, q), r), s))| {
                            let k = idx * 4;
                            let bvec0 = f64x4::from_slice(&tb[j * n + k..]);
                            let bvec1 =
                                f64x4::from_slice(&tb[(j + 1) * n + k..]);
                            let bvec2 =
                                f64x4::from_slice(&tb[(j + 2) * n + k..]);
                            let bvec3 =
                                f64x4::from_slice(&tb[(j + 3) * n + k..]);

                            let mut avec0 = f64x4::from_slice(p);

                            ans00 += avec0 * bvec0;
                            ans01 += avec0 * bvec1;
                            ans02 += avec0 * bvec2;
                            ans03 += avec0 * bvec3;

                            avec0 = f64x4::from_slice(q);

                            ans10 += avec0 * bvec0;
                            ans11 += avec0 * bvec1;
                            ans12 += avec0 * bvec2;
                            ans13 += avec0 * bvec3;

                            avec0 = f64x4::from_slice(r);

                            ans20 += avec0 * bvec0;
                            ans21 += avec0 * bvec1;
                            ans22 += avec0 * bvec2;
                            ans23 += avec0 * bvec3;

                            avec0 = f64x4::from_slice(s);

                            ans30 += avec0 * bvec0;
                            ans31 += avec0 * bvec1;
                            ans32 += avec0 * bvec2;
                            ans33 += avec0 * bvec3;
                        });

                    c[i * p + j] += ans00.reduce_sum();
                    c[i * p + j + 1] += ans01.reduce_sum();
                    c[i * p + j + 2] += ans02.reduce_sum();
                    c[i * p + j + 3] += ans03.reduce_sum();

                    c[(i + 1) * p + j] += ans10.reduce_sum();
                    c[(i + 1) * p + j + 1] += ans11.reduce_sum();
                    c[(i + 1) * p + j + 2] += ans12.reduce_sum();
                    c[(i + 1) * p + j + 3] += ans13.reduce_sum();

                    c[(i + 2) * p + j] += ans20.reduce_sum();
                    c[(i + 2) * p + j + 1] += ans21.reduce_sum();
                    c[(i + 2) * p + j + 2] += ans22.reduce_sum();
                    c[(i + 2) * p + j + 3] += ans23.reduce_sum();

                    c[(i + 3) * p + j] += ans30.reduce_sum();
                    c[(i + 3) * p + j + 1] += ans31.reduce_sum();
                    c[(i + 3) * p + j + 2] += ans32.reduce_sum();
                    c[(i + 3) * p + j + 3] += ans33.reduce_sum();
                }
            }
        }
    }

    c
}
