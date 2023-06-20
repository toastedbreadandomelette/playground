#![feature(stdsimd)]
#![feature(portable_simd)]
#![feature(array_windows)]
mod matmul;
mod matmul_2;

fn main() {
    let sz = 1024;
    let a = (0..sz * sz).map(|c| c as f64).collect::<Vec<f64>>();
    let mut b = (0..sz * sz).map(|c| c as f64).collect::<Vec<f64>>();

    let mut t = std::time::Instant::now();
    let orig = matmul::matmul(&a, &b, (sz, sz), (sz, sz));
    println!("Naive: {}ms", t.elapsed().as_millis());

    t = std::time::Instant::now();
    let mut c = unsafe { matmul_2::matmul_transposed_simd_accumulated_4x4(&a, &mut b, (sz, sz), (sz, sz)) };
    println!(
        "Transposed and simd-accumulated 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );
    // let tmpa = a.into_boxed_slice();
    // let tmpb = b.into_boxed_slice();
    let ba: &[f64] = &a[..];
    let bb: &[f64] = &b[..];

    t = std::time::Instant::now();
    c = unsafe { matmul_2::matmul_transposed_multi_accumulated_simd_4x4(&ba, &bb, (sz, sz), (sz, sz)) };
    println!(
        "Transposed and multi accumulated simd 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = unsafe { 
        matmul_2::cf_block_transposed_multi_accumulated_simd_matmul_4x4(
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

    // t = std::time::Instant::now();
    // c = matmul::cf_block_matmul_simd(&a, &b, (sz, sz), (sz, sz));
    // println!(
    //     "Cache friendly blocked simd {}ms, {}",
    //     t.elapsed().as_millis(),
    //     orig == c
    // );

    // t = std::time::Instant::now();
    // c = matmul::cf_block_matmul_alternate(&a, &b, (sz, sz), (sz, sz));
    // println!(
    //     "Cache friendly blocked {}ms, {}",
    //     t.elapsed().as_millis(),
    //     orig == c
    // );
}
