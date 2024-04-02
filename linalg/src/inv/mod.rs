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

pub fn display_mat(arr: &[f64], n: usize) {
    arr.chunks_exact(n).for_each(|c| println!("{:?}", c));
    println!();
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

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn div2(
    acbr0: &mut [f64],
    invbr0: &mut [f64],
    acbr1: &mut [f64],
    invbr1: &mut [f64],
    acr: &[f64],
    invr: &[f64],
    factor0: f64,
    factor1: f64,
) {
    let value0 = f64x4::splat(factor0);
    let value1 = f64x4::splat(factor1);

    let prod_iter = acr
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(invr.chunks_exact(4).map(f64x4::from_slice))
        .map(|(acre, invre)| (acre, invre));

    acbr0
        .chunks_exact_mut(4)
        .zip(invbr0.chunks_exact_mut(4))
        .zip(acbr1.chunks_exact_mut(4).zip(invbr1.chunks_exact_mut(4)))
        .zip(prod_iter)
        .for_each(|(((acbre0, invbre0), (acbre1, invbre1)), (acre, invre))| {
            let acbres0 = f64x4::from_slice(acbre0);
            let acbres1 = f64x4::from_slice(acbre1);

            (acbres0 - value0 * acre).copy_to_slice(acbre0);
            (acbres1 - value1 * acre).copy_to_slice(acbre1);

            let invbres0 = f64x4::from_slice(invbre0);
            let invbres1 = f64x4::from_slice(invbre1);

            (invbres0 - value0 * invre).copy_to_slice(invbre0);
            (invbres1 - value1 * invre).copy_to_slice(invbre1);
        });
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn div3(
    acbr0: &mut [f64],
    invbr0: &mut [f64],
    acbr1: &mut [f64],
    invbr1: &mut [f64],
    acbr2: &mut [f64],
    invbr2: &mut [f64],
    acr: &[f64],
    invr: &[f64],
    factor0: f64,
    factor1: f64,
    factor2: f64,
) {
    let (value0, value1, value2) = (
        f64x4::splat(factor0),
        f64x4::splat(factor1),
        f64x4::splat(factor2),
    );

    let prod_iter = acr
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(invr.chunks_exact(4).map(f64x4::from_slice))
        .map(|(acre, invre)| (acre, invre));

    acbr0
        .chunks_exact_mut(4)
        .zip(invbr0.chunks_exact_mut(4))
        .zip(acbr1.chunks_exact_mut(4).zip(invbr1.chunks_exact_mut(4)))
        .zip(acbr2.chunks_exact_mut(4).zip(invbr2.chunks_exact_mut(4)))
        .zip(prod_iter)
        .for_each(
            |(
                (((acbre0, invbre0), (acbre1, invbre1)), (acbre2, invbre2)),
                (acre, invre),
            )| {
                let acbres0 = f64x4::from_slice(acbre0);
                let acbres1 = f64x4::from_slice(acbre1);
                let acbres2 = f64x4::from_slice(acbre2);

                (acbres0 - value0 * acre).copy_to_slice(acbre0);
                (acbres1 - value1 * acre).copy_to_slice(acbre1);
                (acbres2 - value2 * acre).copy_to_slice(acbre2);

                let invbres0 = f64x4::from_slice(invbre0);
                let invbres1 = f64x4::from_slice(invbre1);
                let invbres2 = f64x4::from_slice(invbre2);

                (invbres0 - value0 * invre).copy_to_slice(invbre0);
                (invbres1 - value1 * invre).copy_to_slice(invbre1);
                (invbres2 - value2 * invre).copy_to_slice(invbre2);
            },
        );
}

#[inline]
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,avx2,fma")]
pub unsafe fn div4(
    acbr0: &mut [f64],
    invbr0: &mut [f64],
    acbr1: &mut [f64],
    invbr1: &mut [f64],
    acbr2: &mut [f64],
    invbr2: &mut [f64],
    acbr3: &mut [f64],
    invbr3: &mut [f64],
    acr: &[f64],
    invr: &[f64],
    factor0: f64,
    factor1: f64,
    factor2: f64,
    factor3: f64,
) {
    let (value0, value1, value2, value3) = (
        f64x4::splat(factor0),
        f64x4::splat(factor1),
        f64x4::splat(factor2),
        f64x4::splat(factor3),
    );

    let prod_iter = acr
        .chunks_exact(4)
        .map(f64x4::from_slice)
        .zip(invr.chunks_exact(4).map(f64x4::from_slice))
        .map(|(acre, invre)| (acre, invre));

    acbr0
        .chunks_exact_mut(4)
        .zip(invbr0.chunks_exact_mut(4))
        .zip(acbr1.chunks_exact_mut(4).zip(invbr1.chunks_exact_mut(4)))
        .zip(acbr2.chunks_exact_mut(4).zip(invbr2.chunks_exact_mut(4)))
        .zip(acbr3.chunks_exact_mut(4).zip(invbr3.chunks_exact_mut(4)))
        .zip(prod_iter)
        .for_each(
            |(
                (
                    (((acbre0, invbre0), (acbre1, invbre1)), (acbre2, invbre2)),
                    (acbre3, invbre3),
                ),
                (acre, invre),
            )| {
                let acbres0 = f64x4::from_slice(acbre0);
                let acbres1 = f64x4::from_slice(acbre1);
                let acbres2 = f64x4::from_slice(acbre2);
                let acbres3 = f64x4::from_slice(acbre3);

                (acbres0 - value0 * acre).copy_to_slice(acbre0);
                (acbres1 - value1 * acre).copy_to_slice(acbre1);
                (acbres2 - value2 * acre).copy_to_slice(acbre2);
                (acbres3 - value3 * acre).copy_to_slice(acbre3);

                let invbres0 = f64x4::from_slice(invbre0);
                let invbres1 = f64x4::from_slice(invbre1);
                let invbres2 = f64x4::from_slice(invbre2);
                let invbres3 = f64x4::from_slice(invbre3);

                (invbres0 - value0 * invre).copy_to_slice(invbre0);
                (invbres1 - value1 * invre).copy_to_slice(invbre1);
                (invbres2 - value2 * invre).copy_to_slice(invbre2);
                (invbres3 - value3 * invre).copy_to_slice(invbre3);
            },
        );
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
    // let check = crate::matmul::cf_blocked_simd::cf_blocked_simd(&a, &some, (sz, sz), (sz, sz));

    // display_mat(&check, sz);
    println!(
        "{}",
        some.iter().zip(orig.iter()).all(|(a, b)| close_to(*a, *b))
    );

    let t = std::time::Instant::now();
    let some = inv_iter_simd::inv_iter_simd(&a, sz);
    println!("Using Iters SIMD: {}ms", t.elapsed().as_millis());

    // let check = crate::matmul::cf_blocked_simd::cf_blocked_simd(&a, &some, (sz, sz), (sz, sz));
    // display_mat(&check, sz);
    println!(
        "{}",
        some.iter().zip(orig.iter()).all(|(a, b)| close_to(*a, *b))
    );
}
