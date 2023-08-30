use crate::common::{vector::Vector, *};

/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Difference is that the natrix `b` is transposed and
/// transposed matrix is used as a way to multiply.
///
/// Also accumulation is done by computing 4 cells per iteration.
///
/// Returns new matrix vector `c` of size (m x p)
///
/// This can also be stated as `1x4` kernel multiplication
pub fn matmul_transposed_multi_accumulated(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let c: Vector<f64> = Vector::zeroed(m * p);

    // Before computing matrix multiplication:
    // we transpose them
    let (tb, _) = transpose_vec(b, (n, p));

    a.chunks(n).zip(c.chunks_mut(p)).for_each(|(avec, cvec)| {
        tb.chunks_exact(n * 4).zip(cvec.chunks_mut(4)).for_each(|(bvec, cval_vec)| {
            // let j = jl * 4;
            // 4 adjacent are computed simultaneously
            // so that values in row `avec` is used once for multiple
            // mul operations at once, reducing branches
            (
                cval_vec[0],
                cval_vec[1],
                cval_vec[2],
                cval_vec[3],
            ) = dot4(
                &avec,
                &bvec[0..n],
                &bvec[n..2 * n],
                &bvec[2 * n..3 * n],
                &bvec[3 * n..4 * n],
            );
        });
        // Residual operation is done after the above computation
        let val = tb.chunks_exact(n * 4).remainder();
        match val.len() / n {
            1 => *cvec.last_mut().unwrap() = dot(avec, val),
            2 => {
                (cvec[n - 2], cvec[n - 1]) =
                    dot2(avec, &val[0..n], &val[n..2 * n])
            }
            3 => {
                (cvec[n - 3], cvec[n - 2], cvec[n - 1]) =
                    dot3(avec, &val[0..n], &val[n..2 * n], &val[2 * n..3 * n])
            }
            _ => {}
        }
    });
    c
}
