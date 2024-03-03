#![feature(step_trait, array_windows, portable_simd)]
// #![no_std]

mod common;
mod inv;
mod matmul;

fn main() {
    let sz = 2048;
    // matmul::matmul(sz);
    inv::inverse(sz);
}
