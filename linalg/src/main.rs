#![feature(stdsimd)]
#![feature(portable_simd)]
#![feature(array_windows)]
mod matmul;

fn main() {
    let sz = 1024;
    let a = (0..sz * sz).map(|c| c as f64 / 2.0).collect::<Vec<f64>>();
    let b = (0..sz * sz).map(|c| c as f64 / 2.0).collect::<Vec<f64>>();

    let mut t = std::time::Instant::now();
    let orig = matmul::matmul(&a, &b, (sz, sz), (sz, sz));
    println!("Naive: {}ms", t.elapsed().as_millis());

    t = std::time::Instant::now();
    let mut c = matmul::matmul_tp(&a, &b, (sz, sz), (sz, sz));
    println!("transposed: {}ms {}", t.elapsed().as_millis(), orig == c);
    // let tmpa = a.into_boxed_slice();
    // let tmpb = b.into_boxed_slice();
    let ba: &[f64] = &a[..];
    let bb: &[f64] = &b[..];

    t = std::time::Instant::now();
    c = unsafe { 
        matmul::cf_block_transposed_multi_accumulated_simd_matmul_4x4(
            ba,
            bb,
            (sz, sz),
            (sz, sz),
        )
    };
    println!(
        "Cache friendly blocked transposed and multi-accumulated simd 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );
}
