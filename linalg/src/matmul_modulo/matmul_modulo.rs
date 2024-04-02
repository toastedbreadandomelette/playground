use vector::Vector;

use crate::common::transpose_vec;

fn mod_inverse(modval: u64, inv_val: u64) -> u64 {
    // Assume that modval and inv_val are co-prime to each other
    let mut pow = inv_val - 2;
    let (mut res, mut n) = (1, modval);
    while pow > 0 {
        if pow & 1 == 1 {
            res *= n;
        }
        n *= n;
        pow >>= 1;
    }

    res
}

pub fn matmul_modulo(
    a: &[u64],
    b: &[u64],
    (m, n): (usize, usize),
    (_, p): (usize, usize),
    modulo: u64
) -> Vector<u64> {
    let c = Vector::zeroed(m * p);
    let (tb, _) = transpose_vec(b, (n, p));
    
    a.chunks(n).zip(c.chunks_mut(p)).for_each(|(avec, cvec)| {
        tb.chunks(n).zip(cvec).for_each(|(bvec, cval)| {
            *cval = avec
                .iter()
                .zip(bvec)
                .fold(0, |prev, (a1, b1)| {
                    // println!("{prev} {a1} {b1}");
                    (prev % modulo).wrapping_add(((a1 % modulo) * (b1 % modulo)) % modulo) % modulo
                });
        });
    });

    c
}