#![feature(portable_simd)]
#![feature(iter_intersperse)]
use crate::sha::sha::parse_file;

mod sha;

fn main() {
    let t = std::time::Instant::now();
    parse_file("../../ML_AI/ML/Projects/modified_array/Data8277.csv");
    println!("{:?}ms", t.elapsed().as_millis());
}
