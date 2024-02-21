#![feature(portable_simd)]

use crate::fourier_transform::{
    fast_fft::fast_fft, fast_fft::fast_ifft, faster_fft::faster_fft,
    faster_fft::faster_ifft,
};
use crate::utils::{close_to, display_bin, index_generator::IndexGen};
use vector::Vector;

mod cosine_transform;
mod fourier_transform;
mod utils;

fn main() {
    let sz = 1048576 << 2;
    let x = (0..sz).map(|c| c as f64).collect::<Vector<f64>>();

    let t = std::time::Instant::now();
    _ = fast_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    let t = std::time::Instant::now();
    _ = faster_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());
}
