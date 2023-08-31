use crate::common::{
    dot_simd, dot_simd_2, dot_simd_3, dot_simd_4, dot_simd_4x2,
};

/// Iterate on 1 strip vector of `a` with block of vectors `b`
///
/// Taking 4 strips at a time, and evaluating remainder strips later
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
#[allow(non_snake_case)]
pub unsafe fn iter_blocks_on_1xN(
    a_rem: &[f64],
    b_block: &[f64],
    c: &mut [f64],
    ibl: usize,
    jbl: usize,
    n: usize,
    p: usize,
) {
    // Iterate on 4 of these rows of b_block at a time
    let i = ibl;
    let a0 = &a_rem[..];

    b_block
        .chunks_exact(n * 4)
        .enumerate()
        .for_each(|(b4_index, b_4_row)| {
            let j = jbl + (b4_index << 2);

            let (b0, b1, b2, b3) = (
                &b_4_row[..n],
                &b_4_row[n..2 * n],
                &b_4_row[2 * n..3 * n],
                &b_4_row[3 * n..],
            );
            process_1x4_block(a0, b0, b1, b2, b3, c, (i, j), (n, p));
        });
    // Work on remainder b_blocks
    let b_rem = b_block.chunks_exact(n * 4).remainder();

    match b_rem.len() / n {
        1 => {
            process_1x1_block(a0, b_rem, c, (i, p - 1), (n, p));
        }
        2 => {
            let (b0, b1) = (&b_rem[..n], &b_rem[n..]);
            process_1x2_block(a0, b0, b1, c, (i, p - 2), (n, p));
        }
        3 => {
            let (b0, b1, b2) = (&b_rem[..n], &b_rem[n..2 * n], &b_rem[2 * n..]);
            process_1x3_block(a0, b0, b1, b2, c, (i, p - 3), (n, p));
        }
        _ => {}
    }
}

/// Iterate on 2 strip vector of `a` with block of vectors `b`
///
/// Taking 4 strips at a time, and evaluating remainder strips later
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
#[allow(non_snake_case)]
pub unsafe fn iter_blocks_on_2xN(
    a_rem: &[f64],
    b_block: &[f64],
    c: &mut [f64],
    ibl: usize,
    jbl: usize,
    n: usize,
    p: usize,
) {
    // Iterate on 4 of these rows of b_block at a time
    let i = ibl;
    let (a0, a1) = (&a_rem[..n], &a_rem[n..]);
    b_block
        .chunks_exact(n * 4)
        .enumerate()
        .for_each(|(b4_index, b_4_row)| {
            let j = jbl + (b4_index << 2);

            let (b0, b1, b2, b3) = (
                &b_4_row[..n],
                &b_4_row[n..2 * n],
                &b_4_row[2 * n..3 * n],
                &b_4_row[3 * n..],
            );
            process_2x4_block(a0, a1, b0, b1, b2, b3, c, (i, j), (n, p));
        });
    // Work on remainder b_blocks
    let b_rem = b_block.chunks_exact(n * 4).remainder();

    match b_rem.len() / n {
        1 => {
            process_2x1_block(a0, a1, b_rem, c, (i, p - 1), (n, p));
        }
        2 => {
            let (b0, b1) = (&b_rem[..n], &b_rem[n..]);
            process_2x2_block(a0, a1, b0, b1, c, (i, p - 2), (n, p));
        }
        3 => {
            let (b0, b1, b2) = (&b_rem[..n], &b_rem[n..2 * n], &b_rem[2 * n..]);
            process_2x3_block(a0, a1, b0, b1, b2, c, (i, p - 3), (n, p));
        }
        _ => {}
    }
}

/// Iterate on 3 strip vector of `a` with block of vectors `b`
///
/// Taking 4 strips at a time, and evaluating remainder strips later
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
#[allow(non_snake_case)]
pub unsafe fn iter_blocks_on_3xN(
    a_rem: &[f64],
    b_block: &[f64],
    c: &mut [f64],
    ibl: usize,
    jbl: usize,
    n: usize,
    p: usize,
) {
    // Iterate on 4 of these rows of b_block at a time
    let i = ibl;
    let (a0, a1, a2) = (&a_rem[..n], &a_rem[n..2 * n], &a_rem[2 * n..]);
    b_block
        .chunks_exact(n * 4)
        .enumerate()
        .for_each(|(b4_index, b_4_row)| {
            let j = jbl + (b4_index << 2);

            let (b0, b1, b2, b3) = (
                &b_4_row[..n],
                &b_4_row[n..2 * n],
                &b_4_row[2 * n..3 * n],
                &b_4_row[3 * n..],
            );
            process_3x4_block(a0, a1, a2, b0, b1, b2, b3, c, (i, j), (n, p));
        });
    // Work on remainder b_blocks
    let b_rem = b_block.chunks_exact(n * 4).remainder();

    match b_rem.len() / n {
        1 => {
            process_3x1_block(a0, a1, a2, b_rem, c, (i, p - 1), (n, p));
        }
        2 => {
            let (b0, b1) = (&b_rem[..n], &b_rem[n..]);
            process_3x2_block(a0, a1, a2, b0, b1, c, (i, p - 2), (n, p));
        }
        3 => {
            let (b0, b1, b2) = (&b_rem[..n], &b_rem[n..2 * n], &b_rem[2 * n..]);
            process_3x3_block(a0, a1, a2, b0, b1, b2, c, (i, p - 3), (n, p));
        }
        _ => {}
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
#[allow(non_snake_case)]
pub unsafe fn iter_blocks_on_4xN(
    a_block: &[f64],
    b_block: &[f64],
    c: &mut [f64],
    ibl: usize,
    jbl: usize,
    n: usize,
    p: usize,
) {
    a_block
        .chunks_exact(n * 4)
        .enumerate()
        .for_each(|(a4_index, a_4_row)| {
            // Iterate on 4 of these rows of b_block at a time
            let i = ibl + (a4_index << 2);
            let (a0, a1, a2, a3) = (
                &a_4_row[..n],
                &a_4_row[n..2 * n],
                &a_4_row[2 * n..3 * n],
                &a_4_row[3 * n..],
            );
            b_block.chunks_exact(n * 4).enumerate().for_each(
                |(b4_index, b_4_row)| {
                    let j = jbl + (b4_index << 2);

                    let (b0, b1, b2, b3) = (
                        &b_4_row[0..n],
                        &b_4_row[n..2 * n],
                        &b_4_row[2 * n..3 * n],
                        &b_4_row[3 * n..],
                    );
                    process_4x4_block(
                        a0,
                        a1,
                        a2,
                        a3,
                        b0,
                        b1,
                        b2,
                        b3,
                        c,
                        (i, j),
                        (n, p),
                    );
                },
            );
            // Work on remainder b_blocks
            let b_rem = b_block.chunks_exact(n * 4).remainder();

            match b_rem.len() / n {
                1 => {
                    process_4x1_block(
                        a0,
                        a1,
                        a2,
                        a3,
                        b_rem,
                        c,
                        (i, p - 1),
                        (n, p),
                    );
                }
                2 => {
                    let (b0, b1) = (&b_rem[..n], &b_rem[n..]);
                    process_4x2_block(
                        a0,
                        a1,
                        a2,
                        a3,
                        b0,
                        b1,
                        c,
                        (i, p - 2),
                        (n, p),
                    );
                }
                3 => {
                    let (b0, b1, b2) =
                        (&b_rem[..n], &b_rem[n..2 * n], &b_rem[2 * n..]);
                    process_4x3_block(
                        a0,
                        a1,
                        a2,
                        a3,
                        b0,
                        b1,
                        b2,
                        c,
                        (i, p - 3),
                        (n, p),
                    );
                }
                _ => {}
            }
        });
}

/// Processing 1x1 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_1x1_block(
    a0: &[f64],
    b0: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    c[i * p + j] = dot_simd(a0, b0);
}

/// Processing 2x1 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_2x1_block(
    a0: &[f64],
    a1: &[f64],
    b0: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[(i + 1) * p + j]) = dot_simd_2(b0, a0, a1);
}

/// Processing 3x1 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_3x1_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    b0: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[(i + 1) * p + j], c[(i + 2) * p + j]) =
        dot_simd_3(b0, a0, a1, a2);
}

/// Processing 4x1 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_4x1_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    a3: &[f64],
    b0: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[(i + 1) * p + j],
        c[(i + 2) * p + j],
        c[(i + 3) * p + j],
    ) = dot_simd_4(b0, a0, a1, a2, a3);
}

/// Processing 1x2 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_1x2_block(
    a0: &[f64],
    b0: &[f64],
    b1: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[i * p + j + 1]) = dot_simd_2(a0, b0, b1);
}

/// Processing 2x2 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_2x2_block(
    a0: &[f64],
    a1: &[f64],
    b0: &[f64],
    b1: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[i * p + j + 1]) = dot_simd_2(a0, b0, b1);
    (c[(i + 1) * p + j], c[(i + 1) * p + j + 1]) = dot_simd_2(a1, b0, b1);
}

/// Processing 3x2 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_3x2_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    b0: &[f64],
    b1: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[(i + 1) * p + j], c[(i + 2) * p + j]) =
        dot_simd_3(b0, a0, a1, a2);

    (
        c[(i) * p + j + 1],
        c[(i + 1) * p + j + 1],
        c[(i + 2) * p + j + 1],
    ) = dot_simd_3(b1, a0, a1, a2);
}

/// Processing 4x2 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_4x2_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    a3: &[f64],
    b0: &[f64],
    b1: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[(i + 1) * p + j],
        c[(i + 2) * p + j],
        c[(i + 3) * p + j],
        c[i * p + j + 1],
        c[(i + 1) * p + j + 1],
        c[(i + 2) * p + j + 1],
        c[(i + 3) * p + j + 1],
    ) = dot_simd_4x2(b0, b1, a0, a1, a2, a3);
}

/// Processing 1x3 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_1x3_block(
    a0: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[i * p + j + 1], c[i * p + j + 2]) =
        dot_simd_3(a0, b0, b1, b2);
}

/// Processing 2x3 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_2x3_block(
    a0: &[f64],
    a1: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[i * p + j + 1], c[i * p + j + 2]) =
        dot_simd_3(a0, b0, b1, b2);

    (
        c[(i + 1) * p + j],
        c[(i + 1) * p + j + 1],
        c[(i + 1) * p + j + 2],
    ) = dot_simd_3(a1, b0, b1, b2);
}

/// Processing 3x3 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_3x3_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (c[i * p + j], c[i * p + j + 1], c[i * p + j + 2]) =
        dot_simd_3(a0, b0, b1, b2);

    (
        c[(i + 1) * p + j],
        c[(i + 1) * p + j + 1],
        c[(i + 1) * p + j + 2],
    ) = dot_simd_3(a1, b0, b1, b2);

    (
        c[(i + 2) * p + j],
        c[(i + 2) * p + j + 1],
        c[(i + 2) * p + j + 2],
    ) = dot_simd_3(a2, b0, b1, b2);
}

/// Processing 4x3 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_4x3_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    a3: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[(i + 1) * p + j],
        c[(i + 2) * p + j],
        c[(i + 3) * p + j],
        c[i * p + j + 1],
        c[(i + 1) * p + j + 1],
        c[(i + 2) * p + j + 1],
        c[(i + 3) * p + j + 1],
    ) = dot_simd_4x2(b0, b1, a0, a1, a2, a3);

    (
        c[i * p + j + 2],
        c[(i + 1) * p + j + 2],
        c[(i + 2) * p + j + 2],
        c[(i + 3) * p + j + 2],
    ) = dot_simd_4(b2, a0, a1, a2, a3);
}

/// Processing 1x4 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_1x4_block(
    a0: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    b3: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[i * p + j + 1],
        c[i * p + j + 2],
        c[i * p + j + 3],
    ) = dot_simd_4(a0, b0, b1, b2, b3);
}

/// Processing 2x4 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_2x4_block(
    a0: &[f64],
    a1: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    b3: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[i * p + j + 1],
        c[i * p + j + 2],
        c[i * p + j + 3],
        c[(i + 1) * p + j],
        c[(i + 1) * p + j + 1],
        c[(i + 1) * p + j + 2],
        c[(i + 1) * p + j + 3],
    ) = dot_simd_4x2(a0, a1, b0, b1, b2, b3);
}

/// Processing 3x4 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_3x4_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    b3: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[i * p + j + 1],
        c[i * p + j + 2],
        c[i * p + j + 3],
        c[(i + 1) * p + j],
        c[(i + 1) * p + j + 1],
        c[(i + 1) * p + j + 2],
        c[(i + 1) * p + j + 3],
    ) = dot_simd_4x2(a0, a1, b0, b1, b2, b3);

    (
        c[(i + 2) * p + j],
        c[(i + 2) * p + j + 1],
        c[(i + 2) * p + j + 2],
        c[(i + 2) * p + j + 3],
    ) = dot_simd_4(a2, b0, b1, b2, b3);
}

/// Processing 4x4 kernel
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn process_4x4_block(
    a0: &[f64],
    a1: &[f64],
    a2: &[f64],
    a3: &[f64],
    b0: &[f64],
    b1: &[f64],
    b2: &[f64],
    b3: &[f64],
    c: &mut [f64],
    (i, j): (usize, usize),
    (_, p): (usize, usize),
) {
    (
        c[i * p + j],
        c[i * p + j + 1],
        c[i * p + j + 2],
        c[i * p + j + 3],
        c[(i + 1) * p + j],
        c[(i + 1) * p + j + 1],
        c[(i + 1) * p + j + 2],
        c[(i + 1) * p + j + 3],
    ) = dot_simd_4x2(a0, a1, b0, b1, b2, b3);

    (
        c[(i + 2) * p + j],
        c[(i + 2) * p + j + 1],
        c[(i + 2) * p + j + 2],
        c[(i + 2) * p + j + 3],
        c[(i + 3) * p + j],
        c[(i + 3) * p + j + 1],
        c[(i + 3) * p + j + 2],
        c[(i + 3) * p + j + 3],
    ) = dot_simd_4x2(a2, a3, b0, b1, b2, b3);
}
