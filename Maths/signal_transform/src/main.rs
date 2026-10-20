#![feature(portable_simd)]

use crate::{fourier_transform::{fast_fft::fast_fft, faster_fft::faster_fft}, utils::index_generator::IndexGen};
use utils::c64::C64;
use vector::Vector;

mod cosine_transform;
mod fourier_transform;
mod utils;

fn main() {
    let c = IndexGen::new(64);
    c.into_iter().for_each(|f| print!("{}, ", f));
    println!("");
    let sz = 1048576 << 3;
    let x = (0..sz).map(|c| c as f64).collect::<Vector<f64>>();

    let t = std::time::Instant::now();
    _ = fast_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    let t = std::time::Instant::now();
    _ = faster_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());
}
