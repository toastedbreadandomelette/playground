use crate::common::transpose_vec;
use crate::matmul::cf_process_blocks::{
    iter_blocks_on_1xN, iter_blocks_on_2xN, iter_blocks_on_3xN,
    iter_blocks_on_NxN,
};
use vector::Vector;

const R_BLOCKSIZE: usize = 32;
const C_BLOCKSIZE: usize = 32;

/// Cache friendly and blocked matrix multiplication of two matrices
/// `a` and `b` of shape `ashape (m x n)` and `bshape (n x p)` respectively
///
/// Uses SIMD functions: calls unsafe functions internally for computation
pub fn cf_blocked_simd(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    unsafe { cf_blocked_simd_unsafe(a, b, ashape, bshape) }
}

/// - Todo: 
/// - [ ] Perform the task multi-threaded (probably rayon `par_iter`)
/// - [ ] Use alternative sub-sizes (E.g, `2x5`, `4x4`, etc.)
/// 
/// Matrix multiplication:
/// Computes values of `4x4` sub-matrix of `c`.
/// Current implementation supports only 
/// 16 dot products at a time.
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
