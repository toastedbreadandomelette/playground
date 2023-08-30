use crate::common::{transpose_vec, dot_simd_4};
use crate::common::vector::Vector;
use core::simd::f64x4;
use core::simd::SimdFloat;

const BLOCKSIZE: usize = 64;

pub fn cf_blocked_simd(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    unsafe { cf_blocked_simd_unsafe(a, b, ashape, bshape) }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn cf_blocked_simd_unsafe(
    a: &[f64],
    b: &[f64],
    (m, n): (usize, usize),
    (_, p): (usize, usize),
) -> Vector<f64> {

    let mut c = Vector::zeroed(m * p);
    let (tb, _) = transpose_vec(b, (n, p));
    let block_size = BLOCKSIZE;

    // let irem = m % block_size;
    // let jrem = p % block_size;

    // let (mut ans00, mut ans01, mut ans02, mut ans03);
    // let (mut ans10, mut ans11, mut ans12, mut ans13);
    // let (mut ans20, mut ans21, mut ans22, mut ans23);
    // let (mut ans30, mut ans31, mut ans32, mut ans33);

    a.chunks(n * block_size)
        .enumerate()
        .for_each(|(i_index, a_block)| {
            let ibl = i_index * block_size;
            tb.chunks(n * block_size).enumerate().for_each(
                |(j_index, b_block)| {
                    let jbl = j_index * block_size;
                    a_block.chunks_exact(n * 4).enumerate().for_each(
                        |(a4_index, a_4_row)| {
                            b_block.chunks_exact(n * 4).enumerate().for_each(
                                |(b4_index, b_4_row)| {
                                    let (i, j) = (
                                        ibl + (a4_index << 2),
                                        jbl + (b4_index << 2),
                                    );
                                    let (a0, a1, a2, a3) = (
                                        &a_4_row[0..n],
                                        &a_4_row[n..2 * n],
                                        &a_4_row[2 * n..3 * n],
                                        &a_4_row[3 * n..],
                                    );
                                    let (b0, b1, b2, b3) = (
                                        &b_4_row[0..n],
                                        &b_4_row[n..2 * n],
                                        &b_4_row[2 * n..3 * n],
                                        &b_4_row[3 * n..],
                                    );
                                    (
                                        c[i * p + j],
                                        c[i * p + j + 1],
                                        c[i * p + j + 2],
                                        c[i * p + j + 3],
                                    ) = dot_simd_4(a0, b0, b1, b2, b3);

                                    (
                                        c[(i + 1) * p + j],
                                        c[(i + 1) * p + j + 1],
                                        c[(i + 1) * p + j + 2],
                                        c[(i + 1) * p + j + 3],
                                    ) = dot_simd_4(a1, b0, b1, b2, b3);

                                    (
                                        c[(i + 2) * p + j],
                                        c[(i + 2) * p + j + 1],
                                        c[(i + 2) * p + j + 2],
                                        c[(i + 2) * p + j + 3],
                                    ) = dot_simd_4(a2, b0, b1, b2, b3);

                                    (
                                        c[(i + 3) * p + j],
                                        c[(i + 3) * p + j + 1],
                                        c[(i + 3) * p + j + 2],
                                        c[(i + 3) * p + j + 3],
                                    ) = dot_simd_4(a3, b0, b1, b2, b3);
                                },
                            );
                        },
                    );
                },
            );
        });

    c
}

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
