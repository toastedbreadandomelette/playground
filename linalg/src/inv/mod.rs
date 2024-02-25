pub mod inv_iter;
pub mod inv_iter_simd;
pub mod inv_normal;

use crate::common::close_to;
#[allow(unused)]
use core::simd::{f64x4, Simd};
use rand::Rng;
use vector::Vector;

#[allow(unused)]
#[inline]
pub fn check_inverse(a: &[f64], n: usize) -> bool {
    a.iter()
        .enumerate()
        .all(|(i, el)| close_to(*el, if i % (n + 1) == 0 { 1.0 } else { 0.0 }))
}

#[inline]
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn div(
    acbr: &mut [f64],
    invbr: &mut [f64],
    acr: &[f64],
    invr: &[f64],
    factor: f64,
) {
    let value = f64x4::splat(factor);

    let prod_iter = acr
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(invr.chunks_exact(4).map(f64x4::from_slice))
        .map(|(acre, invre)| (value * acre, value * invre));

    acbr.chunks_exact_mut(4)
        .zip(invbr.chunks_exact_mut(4))
        .zip(prod_iter)
        .for_each(|((acbre, invbre), (acre_res, invre_res))| {
            let acbres = f64x4::from_slice(acbre);
            let invbres = f64x4::from_slice(invbre);
            (acbres - acre_res).copy_to_slice(acbre);
            (invbres - invre_res).copy_to_slice(invbre);
        });
}

pub fn inverse(sz: usize) {
    let mut rng = rand::thread_rng();
    let a: Vector<f64> = (0..sz * sz)
        .map(|_| ((rng.gen::<f64>().abs()) * (sz as f64)).floor())
        .collect();

    let t = std::time::Instant::now();
    let orig = inv_normal::inv_normal(&a, sz);
    println!("Naive: {}ms", t.elapsed().as_millis());

    let t = std::time::Instant::now();
    let some = inv_iter::inv_iter(&a, sz);
    println!("Using Iters: {}ms", t.elapsed().as_millis());
    // let check = crate::matmul::cf_blocked_simd::cf_blocked_simd(&a, &orig, (sz, sz), (sz, sz));

    println!(
        "{}",
        some.iter().zip(orig.iter()).all(|(a, b)| close_to(*a, *b))
    );

    let t = std::time::Instant::now();
    let some = inv_iter_simd::inv_iter_simd(&a, sz);
    println!("Using Iters: {}ms", t.elapsed().as_millis());

    // display_mat(&some, sz);
    println!(
        "{}",
        some.iter().zip(orig.iter()).all(|(a, b)| close_to(*a, *b))
    );
}
