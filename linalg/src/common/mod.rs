use core::simd::{f64x4, Simd};
use vector::Vector;

#[inline(always)]
pub fn close_to(o: f64, a: f64) -> bool {
    (o - a).abs() < 1e-6 + 1e-6 * a.abs()
}
/// Internal: Dot product of two vectors.
///
/// Returns the value
#[inline(always)]
pub fn dot(avec: &[f64], bvec: &[f64]) -> f64 {
    avec.iter()
        .zip(bvec)
        .fold(0.0, |prev, (a1, b1)| prev + (a1 * b1))
}

/// Reduce 4-way lane SIMD value to one.
///
/// Returns f64
#[inline(always)]
pub fn reduce_sum(asimd: Simd<f64, 4>) -> f64 {
    asimd.as_array().iter().fold(0.0, |p, c| p + c)
}

/// Internal: Dot SIMD product of vector `a` and `b`.
///
/// Returns the value
#[inline]
pub fn dot_simd(avec: &[f64], bvec0: &[f64]) -> f64 {
    let pre0 = avec
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .fold(f64x4::splat(0.0), |prev0, (a, b0)| (prev0 + a * b0));

    let (posta, postb0) = (
        avec.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
    );

    reduce_sum(pre0) + dot(posta, postb0)
}

/// Internal: Dot SIMD product of two simultaneous vectors.
///
/// Returns the value
#[inline]
pub fn dot_simd_2(avec: &[f64], bvec0: &[f64], bvec1: &[f64]) -> (f64, f64) {
    let (pre0, pre1) = avec
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (f64x4::splat(0.0), f64x4::splat(0.0)),
            |(prev0, prev1), ((a, b0), b1)| (prev0 + a * b0, prev1 + a * b1),
        );

    let (posta, postb0, postb1) = (
        avec.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta, postb0),
        reduce_sum(pre1) + dot(posta, postb1),
    )
}

/// Internal: Dot SIMD product of three simultaneous vectors.
///
/// Returns the value
#[inline]
pub fn dot_simd_3(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
) -> (f64, f64, f64) {
    let (pre0, pre1, pre2) = avec
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (f64x4::splat(0.0), f64x4::splat(0.0), f64x4::splat(0.0)),
            |(prev0, prev1, prev2), (((a, b0), b1), b2)| {
                (prev0 + a * b0, prev1 + a * b1, prev2 + a * b2)
            },
        );

    let (posta, postb0, postb1, postb2) = (
        avec.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta, postb0),
        reduce_sum(pre1) + dot(posta, postb1),
        reduce_sum(pre2) + dot(posta, postb2),
    )
}

/// Internal: Dot SIMD product of three simultaneous vectors.
///
/// Returns the value
#[inline]
pub fn dot_simd_2x3(
    avec0: &[f64],
    avec1: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
) -> (f64, f64, f64, f64, f64, f64) {
    let (pre0, pre1, pre2, pre3, pre4, pre5) = avec0
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(avec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
            ),
            |(prev0, prev1, prev2, prev3, prev4, prev5),
             ((((a0, a1), b0), b1), b2)| {
                (
                    prev0 + a0 * b0,
                    prev1 + a0 * b1,
                    prev2 + a0 * b2,
                    prev3 + a1 * b0,
                    prev4 + a1 * b1,
                    prev5 + a1 * b2,
                )
            },
        );

    let (posta0, posta1, postb0, postb1, postb2) = (
        avec0.chunks_exact(4).remainder(),
        avec1.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta0, postb0),
        reduce_sum(pre1) + dot(posta0, postb1),
        reduce_sum(pre2) + dot(posta0, postb2),
        reduce_sum(pre3) + dot(posta1, postb0),
        reduce_sum(pre4) + dot(posta1, postb1),
        reduce_sum(pre5) + dot(posta1, postb2),
    )
}

/// Internal: Dot SIMD product of four simultaneous vectors.
///
/// Returns the value
#[inline]
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn dot_simd_4(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
) -> (f64, f64, f64, f64) {
    let (pre0, pre1, pre2, pre3) = avec
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec3.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
            ),
            |(prev0, prev1, prev2, prev3), ((((a, b0), b1), b2), b3)| {
                (
                    prev0 + a * b0,
                    prev1 + a * b1,
                    prev2 + a * b2,
                    prev3 + a * b3,
                )
            },
        );

    let (posta, postb0, postb1, postb2, postb3) = (
        avec.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
        bvec3.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta, postb0),
        reduce_sum(pre1) + dot(posta, postb1),
        reduce_sum(pre2) + dot(posta, postb2),
        reduce_sum(pre3) + dot(posta, postb3),
    )
}

/// Internal: Dot SIMD product of two against four simultaneous vectors.
///
/// Returns the value
#[inline]
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn dot_simd_2x4(
    avec0: &[f64],
    avec1: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let (pre0, pre1, pre2, pre3, pre4, pre5, pre6, pre7) = avec0
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(avec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec3.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
            ),
            |(prev0, prev1, prev2, prev3, prev4, prev5, prev6, prev7),
             (((((a0, a1), b0), b1), b2), b3)| {
                (
                    prev0 + a0 * b0,
                    prev1 + a0 * b1,
                    prev2 + a0 * b2,
                    prev3 + a0 * b3,
                    prev4 + a1 * b0,
                    prev5 + a1 * b1,
                    prev6 + a1 * b2,
                    prev7 + a1 * b3,
                )
            },
        );

    let (posta0, posta1, postb0, postb1, postb2, postb3) = (
        avec0.chunks_exact(4).remainder(),
        avec1.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
        bvec3.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta0, postb0),
        reduce_sum(pre1) + dot(posta0, postb1),
        reduce_sum(pre2) + dot(posta0, postb2),
        reduce_sum(pre3) + dot(posta0, postb3),
        reduce_sum(pre4) + dot(posta1, postb0),
        reduce_sum(pre5) + dot(posta1, postb1),
        reduce_sum(pre6) + dot(posta1, postb2),
        reduce_sum(pre7) + dot(posta1, postb3),
    )
}

/// Internal: Dot SIMD product of four simultaneous vectors.
///
/// Returns the value
#[inline]
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn dot_simd_4x4(
    avec0: &[f64],
    avec1: &[f64],
    avec2: &[f64],
    avec3: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
) -> (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    let (
        pre0,
        pre1,
        pre2,
        pre3,
        pre4,
        pre5,
        pre6,
        pre7,
        pre8,
        pre9,
        pre10,
        pre11,
        pre12,
        pre13,
        pre14,
        pre15,
    ) = avec0
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(avec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(avec2.chunks_exact(4).map(f64x4::from_slice))
        .zip(avec3.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec3.chunks_exact(4).map(f64x4::from_slice))
        .fold(
            (
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
                f64x4::splat(0.0),
            ),
            |(
                prev0,
                prev1,
                prev2,
                prev3,
                prev4,
                prev5,
                prev6,
                prev7,
                prev8,
                prev9,
                prev10,
                prev11,
                prev12,
                prev13,
                prev14,
                prev15,
            ),
             (((((((a0, a1), a2), a3), b0), b1), b2), b3)| {
                (
                    prev0 + a0 * b0,
                    prev1 + a0 * b1,
                    prev2 + a0 * b2,
                    prev3 + a0 * b3,
                    prev4 + a1 * b0,
                    prev5 + a1 * b1,
                    prev6 + a1 * b2,
                    prev7 + a1 * b3,
                    prev8 + a2 * b0,
                    prev9 + a2 * b1,
                    prev10 + a2 * b2,
                    prev11 + a2 * b3,
                    prev12 + a3 * b0,
                    prev13 + a3 * b1,
                    prev14 + a3 * b2,
                    prev15 + a3 * b3,
                )
            },
        );

    let (posta0, posta1, posta2, posta3, postb0, postb1, postb2, postb3) = (
        avec0.chunks_exact(4).remainder(),
        avec1.chunks_exact(4).remainder(),
        avec2.chunks_exact(4).remainder(),
        avec3.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
        bvec3.chunks_exact(4).remainder(),
    );
    (
        reduce_sum(pre0) + dot(posta0, postb0),
        reduce_sum(pre1) + dot(posta0, postb1),
        reduce_sum(pre2) + dot(posta0, postb2),
        reduce_sum(pre3) + dot(posta0, postb3),
        reduce_sum(pre4) + dot(posta1, postb0),
        reduce_sum(pre5) + dot(posta1, postb1),
        reduce_sum(pre6) + dot(posta1, postb2),
        reduce_sum(pre7) + dot(posta1, postb3),
        reduce_sum(pre8) + dot(posta2, postb0),
        reduce_sum(pre9) + dot(posta2, postb1),
        reduce_sum(pre10) + dot(posta2, postb2),
        reduce_sum(pre11) + dot(posta2, postb3),
        reduce_sum(pre12) + dot(posta3, postb0),
        reduce_sum(pre13) + dot(posta3, postb1),
        reduce_sum(pre14) + dot(posta3, postb2),
        reduce_sum(pre15) + dot(posta3, postb3),
    )
}

/// Internal: Dot product of two vectors, but two are computed at a time.
///
/// Returns the pair (`avec . bvec0`, `avec . bvec1`)
#[inline]
pub fn dot2(avec: &[f64], bvec0: &[f64], bvec1: &[f64]) -> (f64, f64) {
    avec.iter()
        .zip(bvec0)
        .zip(bvec1)
        .fold((0.0, 0.0), |(p0, p1), ((a1, b1), b2)| {
            (p0 + (a1 * b1), p1 + a1 * b2)
        })
}

/// Internal: Dot product of two vectors, but three are computed at a time
///
/// Returns the tuple (`avec . bvec0`, `avec . bvec1`, `avec . bvec2`)
#[inline]
pub fn dot3(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
) -> (f64, f64, f64) {
    avec.iter().zip(bvec0).zip(bvec1).zip(bvec2).fold(
        (0.0, 0.0, 0.0),
        |(p0, p1, p2), (((a1, b1), b2), b3)| {
            (p0 + (a1 * b1), p1 + a1 * b2, p2 + a1 * b3)
        },
    )
}

/// Internal: Dot product of two vectors, but four are computed at a time
///
/// Returns the tuple (`avec . bvec0`, `avec . bvec1`, `avec . bvec2`, `avec . bvec3`)
#[inline]
pub fn dot4(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
) -> (f64, f64, f64, f64) {
    avec.iter()
        .zip(bvec0)
        .zip(bvec1)
        .zip(bvec2)
        .zip(bvec3)
        .fold(
            (0.0, 0.0, 0.0, 0.0),
            |(p0, p1, p2, p3), ((((a1, b1), b2), b3), b4)| {
                (p0 + (a1 * b1), p1 + a1 * b2, p2 + a1 * b3, p3 + a1 * b4)
            },
        )
}

/// Compute transpose of matrix `A`
///
/// Also requires to pass shape of matrix, where
/// `m` is total rows in matrix, and `n` is total columns
/// in matrix
pub fn transpose_vec(
    a: &[f64],
    (m, n): (usize, usize),
) -> (Vector<f64>, (usize, usize)) {
    let mut ta: Vector<f64> = Vector::zeroed(a.len());
    // let rblock: usize = 32;

    a.chunks(n).enumerate().for_each(|(i, avec)| {
        avec.iter().zip(ta.iter_mut().skip(i).step_by(m)).for_each(
            |(aval, ta_val)| {
                *ta_val = *aval;
            },
        );
    });

    (ta, (n, m))
}
