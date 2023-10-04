#![feature(portable_simd)]

use crate::fourier_transform::{discrete_fourier_transform, faster_dft};

mod cosine_transform;
mod fourier_transform;
mod utils;

fn main() {
    // println!("Hello world");
    let x = (0..32768).map(|c| c as f64).collect::<Vec<f64>>();
    // println!("{:?}", x);
    let t = std::time::Instant::now();
    let a = discrete_fourier_transform::dft::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    // let sz = 4096;
    // let inp = (0..sz).map(|x| x as f64).collect();
    // let val = discrete_fourier_transform::dft::<f64>(&inp);
    // let orig: Vec<f64> = discrete_fourier_transform::idft::<f64>(&val);
    // println!("{:?}", orig);

    let t = std::time::Instant::now();
    let b = faster_dft::dft_fast::<f64>(&x);
    println!("{}ms", t.elapsed().as_millis());

    let close = |a: f64, b: f64| (a - b).abs() < 1e-6 + 1e-6 * b.abs();

    let t = std::time::Instant::now();
    let x = faster_dft::idft_fast::<f64>(&b);
    println!("{}ms", t.elapsed().as_millis());

    // println!("{:?}", flt);
}
