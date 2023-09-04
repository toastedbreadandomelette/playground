#![feature(portable_simd)]
#![feature(step_trait)]
// #![feature(array_windows)]
// #![no_std]

extern crate vector;
mod common;
mod inv;
mod matmul;
use vector::vector::Vector;

use crate::inv::inv_normal;
use matmul::*;
use rand::Rng;

fn matmul() {
    let sz = 1024;
    let mut rng = rand::thread_rng();
    let a: Vector<f64> = (0..sz * sz).map(|_| rng.gen::<f64>()).collect();
    let b: Vector<f64> = (0..sz * sz).map(|_| rng.gen::<f64>()).collect();

    let mut t = std::time::Instant::now();
    let orig = matmul_normal::matmul_normal(&a, &b, (sz, sz), (sz, sz));
    println!("Naive: {}ms", t.elapsed().as_millis());

    // let mut t = std::time::Instant::now();
    // let vec_orig = matmul_normal::matmul_normal_vec(&a, &b, (sz, sz), (sz, sz));
    // println!(
    //     "Naive vec: {}ms {}",
    //     t.elapsed().as_millis(),
    //     orig.iter().zip(vec_orig).all(|(a, b)| *a == b)
    // );

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
    c = cf_blocked_simd::cf_blocked_simd(&a, &b, (sz, sz), (sz, sz));

    println!(
        "Cache friendly blocked transposed and multi-accumulated simd Iter 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig.iter().zip(c.iter()).all(|(o, a)| {
            (o - a).abs() < 1e-6 + 1e-6 * a.abs()
        })
    );
}

fn inv() {
    let sz = 5;
    let mut rng = rand::thread_rng();
    let a: Vector<f64> = (1..(sz * sz) + 1)
        .map(|_| ((rng.gen::<f64>() * 50.0) + 1.0).round())
        .collect();
    // a.chunks_exact(sz).for_each(|c| println!("{:?}", c));

    let t = std::time::Instant::now();
    let x = inv_normal::inv_normal(&a, sz);
    println!("Inverse {}ms", t.elapsed().as_millis(),);
    // x.chunks_exact(sz).for_each(|c| println!("{:?}", c));
    println!();
    let mul = matmul::matmul_normal::matmul_normal(&a, &x, (sz, sz), (sz, sz));

    mul.chunks_exact(sz).for_each(|c| println!("{:?}", c));
}

fn main() {
    inv();
}
