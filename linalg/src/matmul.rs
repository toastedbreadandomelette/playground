use core::arch::x86_64;
use core::simd::f64x4;
use std::simd::SimdFloat;

const BLOCKSIZE: usize = 128;

pub fn matmul(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    (c, m, p)
}

/// Vector B is transposed to column major
pub fn matmul_transposed(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * p + i]) = (b[j * p + i], b[i * p + j]);
        }
    }
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                c[i * p + j] += a[i * n + k] * b[j * n + k];
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

pub fn matmul_alternate(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    // i loop
    a.windows(n).step_by(n).enumerate().for_each(|(i, arow)| {
        arow.iter().enumerate().for_each(|(j, _)| {
            b.iter()
                .skip(j)
                .step_by(p)
                .zip(arow.iter())
                .enumerate()
                .for_each(|(_k, (be, ae))| {
                    c[i * p + j] += ae * be;
                })
        })
    });
    (c, m, p)
}

pub fn ikj_matmul(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..m {
        for k in 0..n {
            let x = a[i * n + k];
            for j in 0..p {
                c[i * p + j] += x * b[k * p + j];
            }
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_accumulated(
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
    for i in 0..m {
        for j in 0..p {
            let mut ans = 0.0;
            for k in 0..n {
                ans += a[i * n + k] * b[j * p + k];
            }
            c[i * p + j] = ans;
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_simd_accumulated(
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
    for i in 0..m {
        for j in (0..p).step_by(4) {
            let mut ans = f64x4::splat(0.0);
            for k in 0..n {
                ans += f64x4::splat(a[i * n + k])
                    * f64x4::from_array([
                        b[j * p + k],
                        b[(j + 1) * p + k],
                        b[(j + 2) * p + k],
                        b[(j + 3) * p + k],
                    ]);
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(ans.as_array());
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_multi_simd_accumulated(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let zinit = [0.0; 4];
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    for i in 0..m {
        for j in (0..p).step_by(8) {
            let mut ans: [f64x4; 2] = [f64x4::from_slice(&zinit); 2];
            for k in 0..n {
                let ac = f64x4::splat(a[i * n + k]);
                ans[0] += ac
                    * f64x4::from_array([
                        b[j * p + k],
                        b[(j + 1) * p + k],
                        b[(j + 2) * p + k],
                        b[(j + 3) * p + k],
                    ]);
                ans[1] += ac
                    * f64x4::from_array([
                        b[(j + 4) * p + k],
                        b[(j + 5) * p + k],
                        b[(j + 6) * p + k],
                        b[(j + 7) * p + k],
                    ]);
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(ans[0].as_array());
            c[i * p + j + 4..i * p + j + 8].copy_from_slice(ans[1].as_array());
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_multi_accumulated(
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
    for i in 0..m {
        for j in (0..p).step_by(4) {
            // working on multiple values in an accumulator
            let mut ans = [0.0; 4];
            for k in 0..n {
                let av = a[i * n + k];
                ans[0] += av * b[j * p + k];
                ans[1] += av * b[(j + 1) * p + k];
                ans[2] += av * b[(j + 2) * p + k];
                ans[3] += av * b[(j + 3) * p + k];
            }
            c[i * p + j] = ans[0];
            c[i * p + j + 1] = ans[1];
            c[i * p + j + 2] = ans[2];
            c[i * p + j + 3] = ans[3];
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
pub fn ikj_matmul_simd(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let rem = p & 3;
    for i in 0..m {
        let mut crow: Vec<f64x4> = vec![f64x4::splat(0.0); p >> 2];
        for k in 0..n {
            let x = a[i * n + k];
            let d = f64x4::splat(x);
            for j in (0..p - rem).step_by(4) {
                let g = f64x4::from_slice(&b[k * p + j..]);
                crow[j >> 2] += d * g;
            }
            for j in p - rem..p {
                c[i * p + j] += x * b[k * p + j];
            }
        }
        for j in (0..p - rem).step_by(4) {
            c[i * p + j..i * p + j + 4].copy_from_slice(crow[j >> 2].as_array());
        }
    }
    (c, m, p)
}

#[cfg(target_arch = "x86_64")]
pub fn matmul_transposed_accumulated_simd(
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
    let rem = n & 3;
    for i in 0..m {
        for j in 0..p {
            let mut ans = f64x4::splat(0.0);
            // 4 values taken simultaneously
            for k in (0..n - rem).step_by(4) {
                ans += f64x4::from_slice(&a[i * n + k..]) * f64x4::from_slice(&b[j * p + k..]);
            }
            let mut fans = ans.reduce_sum();
            for k in n - rem..n {
                fans += a[i * n + k] * b[j * p + k];
            }
            c[i * p + j] = fans;
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn matmul_transposed_accumulated_simd_8_unrolled(
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
    for i in 0..m {
        for j in 0..p {
            let mut ans: f64x4 = f64x4::splat(0.0);
            for k in (0..n).step_by(32) {
                ans += f64x4::from_slice(&a[i * n + k..]) * f64x4::from_slice(&b[j * p + k..]);
                ans +=
                    f64x4::from_slice(&a[i * n + k + 4..]) * f64x4::from_slice(&b[j * p + k + 4..]);
                ans +=
                    f64x4::from_slice(&a[i * n + k + 8..]) * f64x4::from_slice(&b[j * p + k + 8..]);
                ans += f64x4::from_slice(&a[i * n + k + 12..])
                    * f64x4::from_slice(&b[j * p + k + 12..]);

                ans += f64x4::from_slice(&a[i * n + k + 16..])
                    * f64x4::from_slice(&b[j * p + k + 16..]);
                ans += f64x4::from_slice(&a[i * n + k + 20..])
                    * f64x4::from_slice(&b[j * p + k + 20..]);
                ans += f64x4::from_slice(&a[i * n + k + 24..])
                    * f64x4::from_slice(&b[j * p + k + 24..]);
                ans += f64x4::from_slice(&a[i * n + k + 28..])
                    * f64x4::from_slice(&b[j * p + k + 28..]);
            }
            c[i * p + j] = ans.reduce_sum();
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
pub fn matmul_transposed_multi_accumulated_simd(
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
    let rem = n & 3;
    for i in 0..m {
        for j in (0..p).step_by(4) {
            let mut ans: [f64x4; 4] = [f64x4::splat(0.0); 4];
            // 4 values taken simultaneously
            for k in (0..n - rem).step_by(4) {
                let ac = f64x4::from_slice(&a[i * n + k..]);
                ans[0] += ac * f64x4::from_slice(&b[j * p + k..]);
                ans[1] += ac * f64x4::from_slice(&b[(j + 1) * p + k..]);
                ans[2] += ac * f64x4::from_slice(&b[(j + 2) * p + k..]);
                ans[3] += ac * f64x4::from_slice(&b[(j + 3) * p + k..]);
            }
            let mut fans: [f64; 4] = [0.0; 4];
            fans[0] = ans[0].reduce_sum();
            fans[1] = ans[1].reduce_sum();
            fans[2] = ans[2].reduce_sum();
            fans[3] = ans[3].reduce_sum();
            for k in n - rem..n {
                fans[0] += a[i * n + k] * b[j * n + k];
                fans[1] += a[i * n + k] * b[(j + 1) * n + k];
                fans[2] += a[i * n + k] * b[(j + 2) * n + k];
                fans[3] += a[i * n + k] * b[(j + 3) * n + k];
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(&fans);
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
pub unsafe fn matmul_transposed_multi_accumulated_simd_unrolled_4(
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
    let rem = n & 15;
    for i in 0..m {
        for j in (0..p).step_by(4) {
            let mut ans: [f64x4; 4] = [f64x4::splat(0.0); 4];
            // 4 values taken simultaneously
            for k in (0..n - rem).step_by(16) {
                let mut acc0 = f64x4::from_slice(&a[i * n + k..]);
                // let acc1 = f64x4::from_slice(&a[i * n + k + 4..]);
                // let acc2 = f64x4::from_slice(&a[i * n + k + 8..]);
                // let acc3 = f64x4::from_slice(&a[i * n + k + 12..]);
                ans[0] += acc0 * f64x4::from_slice(&b[j * p + k..]);
                ans[1] += acc0 * f64x4::from_slice(&b[(j + 1) * p + k..]);
                ans[2] += acc0 * f64x4::from_slice(&b[(j + 2) * p + k..]);
                ans[3] += acc0 * f64x4::from_slice(&b[(j + 3) * p + k..]);

                acc0 = f64x4::from_slice(&a[i * n + k + 4..]);
                ans[0] += acc0 * f64x4::from_slice(&b[j * p + k + 4..]);
                ans[1] += acc0 * f64x4::from_slice(&b[(j + 1) * p + k + 4..]);
                ans[2] += acc0 * f64x4::from_slice(&b[(j + 2) * p + k + 4..]);
                ans[3] += acc0 * f64x4::from_slice(&b[(j + 3) * p + k + 4..]);

                acc0 = f64x4::from_slice(&a[i * n + k + 8..]);

                ans[0] += acc0 * f64x4::from_slice(&b[j * p + k + 8..]);
                ans[1] += acc0 * f64x4::from_slice(&b[(j + 1) * p + k + 8..]);
                ans[2] += acc0 * f64x4::from_slice(&b[(j + 2) * p + k + 8..]);
                ans[3] += acc0 * f64x4::from_slice(&b[(j + 3) * p + k + 8..]);

                acc0 = f64x4::from_slice(&a[i * n + k + 12..]);

                ans[0] += acc0 * f64x4::from_slice(&b[j * p + k + 12..]);
                ans[1] += acc0 * f64x4::from_slice(&b[(j + 1) * p + k + 12..]);
                ans[2] += acc0 * f64x4::from_slice(&b[(j + 2) * p + k + 12..]);
                ans[3] += acc0 * f64x4::from_slice(&b[(j + 3) * p + k + 12..]);
            }
            let mut fans: [f64; 4] = [0.0; 4];
            fans[0] = ans[0].reduce_sum();
            fans[1] = ans[1].reduce_sum();
            fans[2] = ans[2].reduce_sum();
            fans[3] = ans[3].reduce_sum();
            for k in n - rem..n {
                fans[0] += a[i * n + k] * b[j * n + k];
                fans[1] += a[i * n + k] * b[(j + 1) * n + k];
                fans[2] += a[i * n + k] * b[(j + 2) * n + k];
                fans[3] += a[i * n + k] * b[(j + 3) * n + k];
            }
            c[i * p + j..i * p + j + 4].copy_from_slice(&fans);
        }
    }
    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * n + i]) = (b[j * p + i], b[i * n + j]);
        }
    }
    (c, m, p)
}

pub fn ikj_matmul_alternate(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    // i loop
    a.windows(n).step_by(n).enumerate().for_each(|(i, arow)| {
        arow.iter()
            .zip(b.windows(p).step_by(p))
            .enumerate()
            .for_each(|(_k, (acell, brow))| {
                brow.iter().enumerate().for_each(|(j, bcell)| {
                    c[i * p + j] += acell * bcell;
                });
            })
    });
    (c, m, p)
}

pub fn cf_block_matmul(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let block_size = BLOCKSIZE;
    for ibl in (0..m).step_by(block_size) {
        let ilim = if ibl + block_size > m {
            m
        } else {
            ibl + block_size
        };
        for kbl in (0..n).step_by(block_size) {
            let klim = if kbl + block_size > m {
                m
            } else {
                kbl + block_size
            };
            for i in ibl..ilim {
                for k in kbl..klim {
                    let x = a[i * n + k];
                    for j in 0..p {
                        c[i * p + j] += x * b[k * p + j];
                    }
                }
            }
        }
    }
    (c, m, p)
}

pub fn cf_block_transposed_matmul(
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
    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
            for i in ibl..ilim {
                for j in jbl..jlim {
                    let mut ans = 0.0;
                    for k in 0..n {
                        ans += a[i * n + k] * b[j * n + k];
                    }
                    c[i * p + j] += ans;
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

pub fn cf_block_transposed_multi_accumulated_matmul(
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
    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
            for i in ibl..ilim {
                for j in (jbl..jlim).step_by(4) {
                    let mut ans = [0.0; 4];
                    for k in 0..n {
                        ans[0] += a[i * n + k] * b[j * n + k];
                        ans[1] += a[i * n + k] * b[(j + 1) * n + k];
                        ans[2] += a[i * n + k] * b[(j + 2) * n + k];
                        ans[3] += a[i * n + k] * b[(j + 3) * n + k];
                    }
                    c[i * p + j] += ans[0];
                    c[i * p + j + 1] += ans[1];
                    c[i * p + j + 2] += ans[2];
                    c[i * p + j + 3] += ans[3];
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

pub fn cf_block_transposed_simd_accumulated_matmul(
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
    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
            for i in ibl..ilim {
                for j in (jbl..jlim).step_by(4) {
                    let mut ans = f64x4::from_slice(&c[i * p + j..]);
                    for k in 0..n {
                        let ac = f64x4::from_slice(&[a[i * n + k]; 4]);
                        ans += ac
                            * f64x4::from_slice(&[
                                b[j * n + k],
                                b[(j + 1) * n + k],
                                b[(j + 2) * n + k],
                                b[(j + 3) * n + k],
                            ]);
                    }
                    c[i * p + j..i * p + j + 4].copy_from_slice(ans.as_array());
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
pub fn cf_block_transposed_accumulated_simd_matmul(
    a: &Vec<f64>,
    b: &mut Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let block_size = 256;

    for i in 0..n {
        for j in (i + 1)..p {
            (b[i * p + j], b[j * p + i]) = (b[j * p + i], b[i * p + j]);
        }
    }
    let irem = m % block_size;
    let jrem = p % block_size;
    let zinit = [0.0; 4];
    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
            for i in ibl..ilim {
                for j in jbl..jlim {
                    let mut ans: f64x4 = f64x4::from_slice(&zinit);
                    for k in (0..n).step_by(4) {
                        let ac = f64x4::from_slice(&a[i * n + k..]);
                        ans += ac * f64x4::from_slice(&b[j * n + k..]);
                    }
                    c[i * p + j] += ans.reduce_sum();
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
pub fn cf_block_transposed_multi_accumulated_simd_matmul(
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
    let zinit = [0.0; 4];
    for ibl in (0..m - irem).step_by(block_size) {
        let ilim = ibl + block_size;
        for jbl in (0..p - jrem).step_by(block_size) {
            let jlim = jbl + block_size;
            for i in ibl..ilim {
                for j in (jbl..jlim).step_by(4) {
                    let mut ans: [f64x4; 4] = [f64x4::from_slice(&zinit); 4];
                    for k in (0..n).step_by(4) {
                        let ac = f64x4::from_slice(&a[i * n + k..]);
                        ans[0] += ac * f64x4::from_slice(&b[j * n + k..]);
                        ans[1] += ac * f64x4::from_slice(&b[(j + 1) * n + k..]);
                        ans[2] += ac * f64x4::from_slice(&b[(j + 2) * n + k..]);
                        ans[3] += ac * f64x4::from_slice(&b[(j + 3) * n + k..]);
                        // ans[4] += ac * f64x4::from_slice(&b[(j + 4) * n + k..]);
                        // ans[5] += ac * f64x4::from_slice(&b[(j + 5) * n + k..]);
                        // ans[6] += ac * f64x4::from_slice(&b[(j + 6) * n + k..]);
                        // ans[7] += ac * f64x4::from_slice(&b[(j + 7) * n + k..]);
                    }
                    c[i * p + j] += ans[0].reduce_sum();
                    c[i * p + j + 1] += ans[1].reduce_sum();
                    c[i * p + j + 2] += ans[2].reduce_sum();
                    c[i * p + j + 3] += ans[3].reduce_sum();
                    // c[i * p + j + 4] += ans[4].reduce_sum();
                    // c[i * p + j + 5] += ans[5].reduce_sum();
                    // c[i * p + j + 6] += ans[6].reduce_sum();
                    // c[i * p + j + 7] += ans[7].reduce_sum();
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

pub fn cf_block_matmul_simd(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> (Vec<f64>, usize, usize) {
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    let block_size = BLOCKSIZE;
    let rem = p & 3;
    for ibl in (0..m).step_by(block_size) {
        let ilim = if ibl + block_size > m {
            m
        } else {
            ibl + block_size
        };
        for kbl in (0..n).step_by(block_size) {
            let klim = if kbl + block_size > m {
                m
            } else {
                kbl + block_size
            };
            for i in ibl..ilim {
                let mut crow: Vec<f64x4> = vec![f64x4::splat(0.0); p >> 2];
                for k in kbl..klim {
                    let x = a[i * n + k];
                    let d = f64x4::splat(x);
                    for j in (0..p - rem).step_by(4) {
                        let g = f64x4::from_slice(&b[k * p + j..]);
                        crow[j >> 2] += d * g;
                    }
                    for j in p - rem..p {
                        c[i * p + j] += x * b[k * p + j];
                    }
                }
                for j in (0..p - rem).step_by(4) {
                    let v = f64x4::from_slice(&c[i * p + j..]);
                    c[i * p + j..i * p + j + 4].copy_from_slice((v + crow[j >> 2]).as_array());
                }
            }
        }
    }
    (c, m, p)
}

#[test]
fn test_matmul() {
    let a = vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0];
    let b = vec![vec![1.0_f64; 3], vec![2.0_f64; 3], vec![3.0_f64; 3]]
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    assert_eq!(matmul(&a, &b, (3, 3), (3, 3)), (vec![14.0; 9], 3, 3));
}
