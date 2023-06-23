use core::simd::f64x4;
use std::simd::SimdFloat;

const BLOCKSIZE: usize = 64;

pub fn matmul(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vec<f64> {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    c
}

pub fn matmul_tp(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vec<f64> {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let mut tmpb: Vec<f64> = vec![0.0; b.len()];

    for i in 0..n {
        for j in 0..p {
            tmpb[j * n + i] = b[i * p + j];
        }
    }

    let tb: &[f64] = tmpb.as_ref();
    a.windows(n).step_by(n).enumerate().for_each(|(i, avec)| {
        tb.windows(n).step_by(n).enumerate().for_each(|(j, bvec)| {
            c[i * p + j] = avec.iter().zip(bvec.iter()).fold(0.0, |prev, (a1, b1)| prev + (a1 * b1));
        });
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
) -> Vec<f64> {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let mut tmpb: Vec<f64> = vec![0.0; b.len()];
    let block_size = BLOCKSIZE;

    for i in 0..n {
        for j in 0..p {
            tmpb[j * n + i] = b[i * p + j];
        }
    }

    let tb: &[f64] = tmpb.as_ref();

    let irem = m % block_size;
    let jrem = p % block_size;
    let krem = n & 3;

    let mut ans00;
    let mut ans01;
    let mut ans02;
    let mut ans03;

    let mut ans10;
    let mut ans11;
    let mut ans12;
    let mut ans13;

    let mut ans20;
    let mut ans21;
    let mut ans22;
    let mut ans23;

    let mut ans30;
    let mut ans31;
    let mut ans32;
    let mut ans33;

    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
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
                            let bvec00 = f64x4::from_slice(&tb[j * n + k..]);
                            let bvec10 = f64x4::from_slice(&tb[(j + 1) * n + k..]);
                            let bvec20 = f64x4::from_slice(&tb[(j + 2) * n + k..]);
                            let bvec30 = f64x4::from_slice(&tb[(j + 3) * n + k..]);

                            let mut avec0 = f64x4::from_slice(p);

                            ans00 += avec0 * bvec00;
                            ans01 += avec0 * bvec10;
                            ans02 += avec0 * bvec20;
                            ans03 += avec0 * bvec30;

                            avec0 = f64x4::from_slice(q);

                            ans10 += avec0 * bvec00;
                            ans11 += avec0 * bvec10;
                            ans12 += avec0 * bvec20;
                            ans13 += avec0 * bvec30;

                            avec0 = f64x4::from_slice(r);

                            ans20 += avec0 * bvec00;
                            ans21 += avec0 * bvec10;
                            ans22 += avec0 * bvec20;
                            ans23 += avec0 * bvec30;

                            avec0 = f64x4::from_slice(s);

                            ans30 += avec0 * bvec00;
                            ans31 += avec0 * bvec10;
                            ans32 += avec0 * bvec20;
                            ans33 += avec0 * bvec30;
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


