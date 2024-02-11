#![feature(portable_simd)]

use crate::fourier_transform::{fast_fft::fast_fft, faster_fft::faster_fft, faster_dft::dft_fast, discrete_fourier_transform::dft};
use vector::Vector;

mod cosine_transform;
mod fourier_transform;
mod utils;

#[inline(always)]
pub fn close_to(o: f64, a: f64) -> bool {
    (o - a).abs() < 1e-4 + 1e-4 * a.abs()
}

fn main() {
    // println!("Hello world");
    let x = (0..32767).map(|c| c as f64).collect::<Vector<f64>>();
    // println!("{:?}", x);
    let t = std::time::Instant::now();
    let a = fast_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    // let sz = 4096;
    // let inp = (0..sz).map(|x| x as f64).collect();
    // let val = discrete_fourier_transform::dft::<f64>(&inp);
    // let orig: Vec<f64> = discrete_fourier_transform::idft::<f64>(&val);
    // println!("{:?}", orig);

    let t = std::time::Instant::now();
    let b = faster_fft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    println!("{:?}", a.iter().zip(b.iter()).all(|(a, b)| close_to(a.real, b.real) && close_to(a.img, b.img)));
    // println!("{:?}", &a[..10]);
    // println!("{:?}", &b[..10]);
}
