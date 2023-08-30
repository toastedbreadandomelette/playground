#![feature(portable_simd)]
#![feature(step_trait)]
// #![no_std]
mod common;
mod matmul;

use crate::common::vector::Vector;
use matmul::*;

fn main() {
    let sz = 1024;
    let a = (0..sz * sz)
        .map(|c| c as f64 / 2.0)
        .collect::<Vector<f64>>();
    let b = (0..sz * sz)
        .map(|c| c as f64 / 2.0)
        .collect::<Vector<f64>>();

    let t = std::time::Instant::now();
    let orig = matmul_normal::matmul_normal(&a, &b, (sz, sz), (sz, sz));
    println!("Naive: {}ms", t.elapsed().as_millis());

    let mut t = std::time::Instant::now();
    let vec_orig = matmul_normal::matmul_normal_vec(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Naive vec: {}ms {}",
        t.elapsed().as_millis(),
        orig.iter().zip(vec_orig).all(|(a, b)| *a == b)
    );

    t = std::time::Instant::now();
    let mut c = matmul_ikj::matmul_ikj(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Reordered matrix multiplication: {}ms {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_transposed::matmul_transposed(&a, &b, (sz, sz), (sz, sz));
    println!("transposed: {}ms {}", t.elapsed().as_millis(), orig == c);

    t = std::time::Instant::now();
    c = matmul_transposed_multi_accumulated::matmul_transposed_multi_accumulated(&a, &b, (sz, sz), (sz, sz));
    println!(
        "transposed and multi accumulated: {}ms {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = unsafe {
        cf_blocked_simd::cf_block_transposed_multi_accumulated_simd_matmul_4x4(
            &a,
            &b,
            (sz, sz),
            (sz, sz),
        )
    };
    println!(
        "Cache friendly blocked transposed and multi-accumulated simd 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = cf_blocked_simd::cf_blocked_simd(&a, &b, (sz, sz), (sz, sz));

    println!(
        "Cache friendly blocked transposed and multi-accumulated simd Iter 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );
}
