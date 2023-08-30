use self::vector::Vector;
use core::simd::{f64x4, Simd};
pub mod array_test;
pub mod vector;

/// Internal: Dot product of two vectors.
///
/// Returns the value
#[inline]
pub fn dot(avec: &[f64], bvec: &[f64]) -> f64 {
    avec.iter()
        .zip(bvec)
        .fold(0.0, |prev, (a1, b1)| prev + (a1 * b1))
}

/// Reduce sun to a single value
#[inline(always)]
pub fn reduce_sum(asimd: &[Simd<f64, 4>], bsimd: &[Simd<f64, 4>]) -> f64 {
    let ans = asimd
        .iter()
        .zip(bsimd)
        .fold(f64x4::splat(0.0), |c, (a, b)| c + a * b);
    ans.as_array().iter().fold(0.0, |p, c| p + c)
}

#[inline(always)]
pub fn red(asimd: Simd<f64, 4>) -> f64 {
    asimd.as_array().iter().fold(0.0, |p, c| p + c)
}

/// Internal: Dot SIMD product of two vectors.
///
/// Returns the value
#[inline]
pub fn dot_simd(avec: &[f64], bvec: &[f64]) -> f64 {
    let (apre, asimd, apost) = avec.as_simd::<4>();
    let (bpre, bsimd, bpost) = bvec.as_simd::<4>();
    apre.iter().zip(bpre).fold(0.0, |c, (a, b)| c + a * b)
        + apost.iter().zip(bpost).fold(0.0, |c, (a, b)| c + a * b)
        + reduce_sum(asimd, bsimd)
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
        red(pre0) + dot(posta, postb0),
        red(pre1) + dot(posta, postb1),
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
        red(pre0) + dot(posta, postb0),
        red(pre1) + dot(posta, postb1),
        red(pre2) + dot(posta, postb2),
    )
}

/// Internal: Dot SIMD product of four simultaneous vectors.
///
/// Returns the value
#[inline]
pub fn dot_simd_4(
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
        red(pre0) + dot(posta, postb0),
        red(pre1) + dot(posta, postb1),
        red(pre2) + dot(posta, postb2),
        red(pre3) + dot(posta, postb3),
    )
}

/// Internal: Dot SIMD product of four simultaneous vectors.
///
/// Returns the value
pub fn dot_simd_8(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
    bvec4: &[f64],
    bvec5: &[f64],
    bvec6: &[f64],
    bvec7: &[f64],
) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let (pre0, pre1, pre2, pre3, pre4, pre5, pre6, pre7) = avec
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(bvec0.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec1.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec2.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec3.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec4.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec5.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec6.chunks_exact(4).map(f64x4::from_slice))
        .zip(bvec7.chunks_exact(4).map(f64x4::from_slice))
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
             ((((((((a, b0), b1), b2), b3), b4), b5), b6), b7)| {
                (
                    prev0 + a * b0,
                    prev1 + a * b1,
                    prev2 + a * b2,
                    prev3 + a * b3,
                    prev4 + a * b4,
                    prev5 + a * b5,
                    prev6 + a * b6,
                    prev7 + a * b7,
                )
            },
        );

    let (posta, postb0, postb1, postb2, postb3, postb4, postb5, postb6, postb7) = (
        avec.chunks_exact(4).remainder(),
        bvec0.chunks_exact(4).remainder(),
        bvec1.chunks_exact(4).remainder(),
        bvec2.chunks_exact(4).remainder(),
        bvec3.chunks_exact(4).remainder(),
        bvec4.chunks_exact(4).remainder(),
        bvec5.chunks_exact(4).remainder(),
        bvec6.chunks_exact(4).remainder(),
        bvec7.chunks_exact(4).remainder(),
    );
    let (d1, d2, d3, d4) = dot4(posta, postb0, postb1, postb2, postb3);
    let (d5, d6, d7, d8) = dot4(posta, postb4, postb5, postb6, postb7);
    (
        red(pre0) + d1,
        red(pre1) + d2,
        red(pre2) + d3,
        red(pre3) + d4,
        red(pre4) + d5,
        red(pre5) + d6,
        red(pre6) + d7,
        red(pre7) + d8,
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
