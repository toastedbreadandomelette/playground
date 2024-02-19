#![feature(portable_simd)]

use crate::fourier_transform::{
    fast_fft::fast_fft, fast_fft::fast_ifft, faster_fft::faster_fft,
    faster_fft::faster_ifft,
};
// use crate::utils::{display_bin, index_generator::IndexGen};
use vector::Vector;

mod cosine_transform;
mod fourier_transform;
mod utils;

#[inline(always)]
pub fn close_to(o: f64, a: f64) -> bool {
    (o - a).abs() < 1e-4 + 1e-4 * o.abs()
}

fn main() {
    let sz = 2097152 + 65536;
    // println!("Hello world");
    let x = (0..sz).map(|c| c as f64).collect::<Vector<f64>>();
    // let mut y: Vector<f64> = Vector::zeroed(sz);
    // println!("{:?}", x);

    let t = std::time::Instant::now();
    let a = fast_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    let t = std::time::Instant::now();
    let b = faster_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    // println!("{:?}", &a[..32]);
    // println!("{:?}", &b[..32]);
    println!(
        "{}",
        a.iter().zip(b.iter()).all(|(a, b)| {
            close_to(a.real, b.real) && close_to(b.img, a.img)
        })
    );

    let t = std::time::Instant::now();
    _ = fast_ifft::<f64>(&a);
    println!("{}ms", t.elapsed().as_millis());

    let t = std::time::Instant::now();
    _ = faster_ifft::<f64>(&b);
    println!("{}ms", t.elapsed().as_millis());
}
