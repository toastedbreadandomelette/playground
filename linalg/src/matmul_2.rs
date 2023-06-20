use core::simd::f64x4;
use std::simd::SimdFloat;
const BLOCKSIZE: usize = 64;

pub fn matmul_transposed_accumulated_4x4(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let mut tb: Vec<f64> = vec![0.0; b.len()];
    for i in 0..n {
        for j in 0..p {
            tb[j * n + i] = b[i * p + j]
        }
    }

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

    for i in (0..m).step_by(4) {
        for j in (0..p).step_by(4) {
            (ans00, ans01, ans02, ans03) = (0.0, 0.0, 0.0, 0.0);
            (ans10, ans11, ans12, ans13) = (0.0, 0.0, 0.0, 0.0);
            (ans20, ans21, ans22, ans23) = (0.0, 0.0, 0.0, 0.0);
            (ans30, ans31, ans32, ans33) = (0.0, 0.0, 0.0, 0.0);
            for k in 0..n {
                let ac0 = a[i * n + k];
                let ac1 = a[(i + 1) * n + k];
                let ac2 = a[(i + 2) * n + k];
                let ac3 = a[(i + 3) * n + k];

                let bc0 = b[j * n + k];
                let bc1 = b[(j + 1) * n + k];
                let bc2 = b[(j + 2) * n + k];
                let bc3 = b[(j + 3) * n + k];
                ans00 += ac0 * bc0;
                ans01 += ac0 * bc1;
                ans02 += ac0 * bc2;
                ans03 += ac0 * bc3;

                ans10 += ac1 * bc0;
                ans11 += ac1 * bc1;
                ans12 += ac1 * bc2;
                ans13 += ac1 * bc3;

                ans20 += ac2 * bc0;
                ans21 += ac2 * bc1;
                ans22 += ac2 * bc2;
                ans23 += ac2 * bc3;

                ans30 += ac3 * bc0;
                ans31 += ac3 * bc1;
                ans32 += ac3 * bc2;
                ans33 += ac3 * bc3;
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(&[ans00, ans01, ans02, ans03]);
            c[(i + 1) * p + j..(i + 1) * p + j + 4].copy_from_slice(&[ans10, ans11, ans12, ans13]);
            c[(i + 2) * p + j..(i + 2) * p + j + 4].copy_from_slice(&[ans20, ans21, ans22, ans23]);
            c[(i + 3) * p + j..(i + 3) * p + j + 4].copy_from_slice(&[ans30, ans31, ans32, ans33]);
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn matmul_transposed_simd_accumulated_4x4(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    for i in (0..m).step_by(4) {
        for j in (0..p).step_by(4) {
            let mut ans0: f64x4 = f64x4::splat(0.0);
            let mut ans1: f64x4 = f64x4::splat(0.0);
            let mut ans2: f64x4 = f64x4::splat(0.0);
            let mut ans3: f64x4 = f64x4::splat(0.0);
            for k in (0..n).step_by(4) {
                let g0 = f64x4::from_array([
                    b[j * p + k],
                    b[(j + 1) * p + k],
                    b[(j + 2) * p + k],
                    b[(j + 3) * p + k],
                ]);
                let g1 = f64x4::from_array([
                    b[j * p + k + 1],
                    b[(j + 1) * p + k + 1],
                    b[(j + 2) * p + k + 1],
                    b[(j + 3) * p + k + 1],
                ]);
                let g2 = f64x4::from_array([
                    b[j * p + k + 2],
                    b[(j + 1) * p + k + 2],
                    b[(j + 2) * p + k + 2],
                    b[(j + 3) * p + k + 2],
                ]);
                let g3 = f64x4::from_array([
                    b[j * p + k + 3],
                    b[(j + 1) * p + k + 3],
                    b[(j + 2) * p + k + 3],
                    b[(j + 3) * p + k + 3],
                ]);
                ans0 += f64x4::splat(a[i * n + k]) * g0;
                ans1 += f64x4::splat(a[(i + 1) * n + k]) * g0;
                ans2 += f64x4::splat(a[(i + 2) * n + k]) * g0;
                ans3 += f64x4::splat(a[(i + 3) * n + k]) * g0;

                ans0 += f64x4::splat(a[i * n + k + 1]) * g1;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 1]) * g1;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 1]) * g1;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 1]) * g1;

                ans0 += f64x4::splat(a[i * n + k + 2]) * g2;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 2]) * g2;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 2]) * g2;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 2]) * g2;

                ans0 += f64x4::splat(a[i * n + k + 3]) * g3;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 3]) * g3;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 3]) * g3;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 3]) * g3;
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(ans0.as_array());
            c[(i + 1) * p + j..(i + 1) * p + j + 4].copy_from_slice(ans1.as_array());
            c[(i + 2) * p + j..(i + 2) * p + j + 4].copy_from_slice(ans2.as_array());
            c[(i + 3) * p + j..(i + 3) * p + j + 4].copy_from_slice(ans3.as_array());
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_simd_accumulated_4x4_unrolled_4(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    for i in (0..m).step_by(4) {
        for j in (0..p).step_by(4) {
            let mut ans0 = f64x4::splat(0.0);
            let mut ans1 = f64x4::splat(0.0);
            let mut ans2 = f64x4::splat(0.0);
            let mut ans3 = f64x4::splat(0.0);
            for k in (0..n).step_by(4) {
                let g0 = f64x4::from_array([
                    b[j * p + k],
                    b[(j + 1) * p + k],
                    b[(j + 2) * p + k],
                    b[(j + 3) * p + k],
                ]);
                let g1 = f64x4::from_array([
                    b[j * p + k + 1],
                    b[(j + 1) * p + k + 1],
                    b[(j + 2) * p + k + 1],
                    b[(j + 3) * p + k + 1],
                ]);
                let g2 = f64x4::from_array([
                    b[j * p + k + 2],
                    b[(j + 1) * p + k + 2],
                    b[(j + 2) * p + k + 2],
                    b[(j + 3) * p + k + 2],
                ]);
                let g3 = f64x4::from_array([
                    b[j * p + k + 3],
                    b[(j + 1) * p + k + 3],
                    b[(j + 2) * p + k + 3],
                    b[(j + 3) * p + k + 3],
                ]);
                ans0 += f64x4::splat(a[i * n + k]) * g0;
                ans1 += f64x4::splat(a[(i + 1) * n + k]) * g0;
                ans2 += f64x4::splat(a[(i + 2) * n + k]) * g0;
                ans3 += f64x4::splat(a[(i + 3) * n + k]) * g0;

                ans0 += f64x4::splat(a[i * n + k + 1]) * g1;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 1]) * g1;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 1]) * g1;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 1]) * g1;

                ans0 += f64x4::splat(a[i * n + k + 2]) * g2;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 2]) * g2;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 2]) * g2;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 2]) * g2;

                ans0 += f64x4::splat(a[i * n + k + 3]) * g3;
                ans1 += f64x4::splat(a[(i + 1) * n + k + 3]) * g3;
                ans2 += f64x4::splat(a[(i + 2) * n + k + 3]) * g3;
                ans3 += f64x4::splat(a[(i + 3) * n + k + 3]) * g3;
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(ans0.as_array());
            c[(i + 1) * p + j..(i + 1) * p + j + 4].copy_from_slice(ans1.as_array());
            c[(i + 2) * p + j..(i + 2) * p + j + 4].copy_from_slice(ans2.as_array());
            c[(i + 3) * p + j..(i + 3) * p + j + 4].copy_from_slice(ans3.as_array());
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}


#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn matmul_transposed_multi_accumulated_simd_4x4(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let mut tb: Vec<f64> = vec![0.0; b.len()];
    
    for i in 0..n {
        for j in 0..p {
            tb[j * n + i] = b[i * p + j]
        }
    }

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

    let rem = n & 3;
    let mut fans: [[f64; 4]; 4] = [[0.0; 4]; 4];
    for i in (0..m).step_by(4) {
        for j in (0..p).step_by(4) {
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
            // 4 values taken simultaneously
            for k in (0..n - rem).step_by(4) {
                let ac0 = f64x4::from_slice(&a[i * n + k..]);
                let ac1 = f64x4::from_slice(&a[(i + 1) * n + k..]);
                let ac2 = f64x4::from_slice(&a[(i + 2) * n + k..]);
                let ac3 = f64x4::from_slice(&a[(i + 3) * n + k..]);

                let bc0 = f64x4::from_slice(&tb[j * n + k..]);
                let bc1 = f64x4::from_slice(&tb[(j + 1) * n + k..]);
                let bc2 = f64x4::from_slice(&tb[(j + 2) * n + k..]);
                let bc3 = f64x4::from_slice(&tb[(j + 3) * n + k..]);

                ans00 += ac0 * bc0;
                ans01 += ac0 * bc1;
                ans02 += ac0 * bc2;
                ans03 += ac0 * bc3;

                ans10 += ac1 * bc0;
                ans11 += ac1 * bc1;
                ans12 += ac1 * bc2;
                ans13 += ac1 * bc3;

                ans20 += ac2 * bc0;
                ans21 += ac2 * bc1;
                ans22 += ac2 * bc2;
                ans23 += ac2 * bc3;

                ans30 += ac3 * bc0;
                ans31 += ac3 * bc1;
                ans32 += ac3 * bc2;
                ans33 += ac3 * bc3;
            }
            fans[0][0] = ans00.reduce_sum();
            fans[0][1] = ans01.reduce_sum();
            fans[0][2] = ans02.reduce_sum();
            fans[0][3] = ans03.reduce_sum();

            fans[1][0] = ans10.reduce_sum();
            fans[1][1] = ans11.reduce_sum();
            fans[1][2] = ans12.reduce_sum();
            fans[1][3] = ans13.reduce_sum();

            fans[2][0] = ans20.reduce_sum();
            fans[2][1] = ans21.reduce_sum();
            fans[2][2] = ans22.reduce_sum();
            fans[2][3] = ans23.reduce_sum();

            fans[3][0] = ans30.reduce_sum();
            fans[3][1] = ans31.reduce_sum();
            fans[3][2] = ans32.reduce_sum();
            fans[3][3] = ans33.reduce_sum();
            // for k in n - rem..n {
            //     fans[0] += a[i * n + k] * b[j * n + k];
            //     fans[1] += a[i * n + k] * b[(j + 1) * n + k];
            //     fans[2] += a[i * n + k] * b[(j + 2) * n + k];
            //     fans[3] += a[i * n + k] * b[(j + 3) * n + k];
            // }
            c[i * p + j..i * p + j + 4].copy_from_slice(&fans[0]);
            c[(i + 1) * p + j..(i + 1) * p + j + 4].copy_from_slice(&fans[1]);
            c[(i + 2) * p + j..(i + 2) * p + j + 4].copy_from_slice(&fans[2]);
            c[(i + 3) * p + j..(i + 3) * p + j + 4].copy_from_slice(&fans[3]);
        }
    }
    (c, m, p)
}

pub fn cf_block_transposed_multi_accumulated_matmul_4x4(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let block_size = BLOCKSIZE;

    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * p + i]) = (b[j * p + i], b[i * p + j]);
        }
    }
    let irem = m % block_size;
    let jrem = p % block_size;

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
                    (ans00, ans01, ans02, ans03) = (0.0, 0.0, 0.0, 0.0);
                    (ans10, ans11, ans12, ans13) = (0.0, 0.0, 0.0, 0.0);
                    (ans20, ans21, ans22, ans23) = (0.0, 0.0, 0.0, 0.0);
                    (ans30, ans31, ans32, ans33) = (0.0, 0.0, 0.0, 0.0);
                    for k in 0..n {
                        let ac0 = a[i * n + k];
                        let ac1 = a[(i + 1) * n + k];
                        let ac2 = a[(i + 2) * n + k];
                        let ac3 = a[(i + 3) * n + k];

                        let bc0 = b[j * n + k];
                        let bc1 = b[(j + 1) * n + k];
                        let bc2 = b[(j + 2) * n + k];
                        let bc3 = b[(j + 3) * n + k];
                        ans00 += ac0 * bc0;
                        ans01 += ac0 * bc1;
                        ans02 += ac0 * bc2;
                        ans03 += ac0 * bc3;

                        ans10 += ac1 * bc0;
                        ans11 += ac1 * bc1;
                        ans12 += ac1 * bc2;
                        ans13 += ac1 * bc3;

                        ans20 += ac2 * bc0;
                        ans21 += ac2 * bc1;
                        ans22 += ac2 * bc2;
                        ans23 += ac2 * bc3;

                        ans30 += ac3 * bc0;
                        ans31 += ac3 * bc1;
                        ans32 += ac3 * bc2;
                        ans33 += ac3 * bc3;
                    }
                    c[i * p + j..i * p + j + 4].copy_from_slice(&[ans00, ans01, ans02, ans03]);
                    c[(i + 1) * p + j..(i + 1) * p + j + 4]
                        .copy_from_slice(&[ans10, ans11, ans12, ans13]);
                    c[(i + 2) * p + j..(i + 2) * p + j + 4]
                        .copy_from_slice(&[ans20, ans21, ans22, ans23]);
                    c[(i + 3) * p + j..(i + 3) * p + j + 4]
                        .copy_from_slice(&[ans30, ans31, ans32, ans33]);
                }
            }
        }
    }

    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * p + i]) = (b[j * p + i], b[i * p + j]);
        }
    }
    (c, m, p)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn cf_block_transposed_multi_accumulated_simd_matmul_4x4(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
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
                    // for k in (0..n).step_by(4) {
                        
                    // }
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

    (c, m, p)
}
