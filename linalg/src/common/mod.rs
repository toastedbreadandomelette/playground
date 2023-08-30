use self::vector::Vector;

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
