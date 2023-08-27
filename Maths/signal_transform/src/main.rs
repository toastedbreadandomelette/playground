#![feature(portable_simd)]

mod cosine_transform;
mod fourier_transform;
mod utils;

fn main() {
    // println!("Hello world");
    let x = (0..8192).map(|c| c as f64).collect::<Vec<f64>>();
    // println!("{:?}", x);
    let t = std::time::Instant::now();
    let _ = cosine_transform::discrete_cosine_transform::dct1::<f64>(&x);
    // println!("{:?}", ans);
    println!("{}ms", t.elapsed().as_millis());
}
