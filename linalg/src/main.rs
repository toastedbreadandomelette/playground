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
    let mut c = matmul::matmul_transposed(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Naive transposed {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_alternate(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Naive alternate {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::ikj_matmul(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Efficient loop ordering {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_accumulated(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::matmul_transposed_accumulated_4x4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and accumulated 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_simd_accumulated(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and simd-accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::matmul_transposed_simd_accumulated_4x4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and simd-accumulated 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::matmul_transposed_simd_accumulated_4x4_unrolled_4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and simd-accumulated 4x4 unrolled by 4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_multi_simd_accumulated(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and multi simd-accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::matmul_transposed_multi_accumulated_simd_4x4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and multi accumulated simd 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_multi_accumulated(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and multi accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::ikj_matmul_alternate(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Efficient loop ordering alternate {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_accumulated_simd(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and accumulated simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_accumulated_simd_8_unrolled(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and accumulated simd unrolled loop by 8 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_multi_accumulated_simd(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and multi accumulated simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::matmul_transposed_multi_accumulated_simd_unrolled_4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Transposed and multi accumulated simd unrolled {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::ikj_matmul_simd(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Efficient loop simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    // t = std::time::Instant::now();
    // c = unsafe { matmul::ikj_matmul_simd_alternate(&a, &b, (sz, sz), (sz, sz)) };
    // println!(
    //     "Efficient loop ordering simd alternate {}ms, {}",
    //     t.elapsed().as_millis(),
    //     orig == c
    // );

    t = std::time::Instant::now();
    c = matmul::cf_block_matmul(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_transposed_matmul(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked and transposed {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_transposed_multi_accumulated_matmul(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked transposed and multi-accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::cf_block_transposed_multi_accumulated_matmul_4x4(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked transposed and multi-accumulated 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_transposed_simd_accumulated_matmul(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked transposed and simd-accumulated {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_transposed_accumulated_simd_matmul(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked transposed and accumulated simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_transposed_multi_accumulated_simd_matmul(&a, &mut b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked transposed and multi-accumulated simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul_2::cf_block_transposed_multi_accumulated_simd_matmul_4x4(
        &a,
        &mut b,
        (sz, sz),
        (sz, sz),
    );
    println!(
        "Cache friendly blocked transposed and multi-accumulated simd 4x4 {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    t = std::time::Instant::now();
    c = matmul::cf_block_matmul_simd(&a, &b, (sz, sz), (sz, sz));
    println!(
        "Cache friendly blocked simd {}ms, {}",
        t.elapsed().as_millis(),
        orig == c
    );

    // t = std::time::Instant::now();
    // c = matmul::cf_block_matmul_alternate(&a, &b, (sz, sz), (sz, sz));
    // println!(
    //     "Cache friendly blocked {}ms, {}",
    //     t.elapsed().as_millis(),
    //     orig == c
    // );
}
