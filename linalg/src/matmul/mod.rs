pub mod cf_blocked_simd;
mod cf_process_blocks;
pub mod matmul_ikj;
pub mod matmul_normal;
pub mod matmul_transposed;
pub mod matmul_transposed_multi_accumulated;

use crate::common::close_to;
use rand::Rng;
use vector::Vector;

pub fn matmul() {
    let sz = 512;
    let mut rng = rand::thread_rng();
    let a: Vector<f64> = (0..sz * sz).map(|_| rng.gen::<f64>()).collect();
    let b: Vector<f64> = (0..sz * sz).map(|_| rng.gen::<f64>()).collect();

    let mut t = std::time::Instant::now();
    let orig = matmul_normal::matmul_normal(&a, &b, (sz, sz), (sz, sz));
    println!("Naive: {}ms", t.elapsed().as_millis());

    t = std::time::Instant::now();
    let mut c = matmul_ikj::matmul_ikj(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Reordered matrix multiplication: {}ms {}",
        t.elapsed().as_millis(),
        orig.iter().zip(&c).all(|(o, a)| close_to(*o, *a))
    );

    t = std::time::Instant::now();
    c = matmul_transposed::matmul_transposed(&a, &b, (sz, sz), (sz, sz));
    println!("transposed: {}ms {}", t.elapsed().as_millis(), orig == c);

    t = std::time::Instant::now();
    c = matmul_transposed_multi_accumulated::matmul_transposed_multi_accumulated(&a, &b, (sz, sz), (sz, sz));
    println!(
        "transposed and multi accumulated: {}ms {} GFLOPS/s {}",
        t.elapsed().as_millis(),
        ((2 * sz * sz * sz) as f64) / (1e6 * t.elapsed().as_millis() as f64),
        orig.iter().zip(&c).all(|(o, a)| close_to(*o, *a))
    );

    t = std::time::Instant::now();
    c = cf_blocked_simd::cf_blocked_simd(&a, &b, (sz, sz), (sz, sz));

    println!(
        "Cache friendly blocked transposed and multi-accumulated simd Iter 4x4 {}ms, {} GFLOPS/s {}",
        t.elapsed().as_millis(),
        ((2 * sz * sz * sz) as f64) / (1e3 * t.elapsed().as_micros() as f64),
        orig.iter().zip(&c).all(|(o, a)| close_to(*o, *a))
    );
}
