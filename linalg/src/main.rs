#![feature(step_trait, array_windows, portable_simd)]
// #![no_std]

mod common;
mod inv;
mod matmul;
use matmul::*;

fn main() {
    inv::inverse();
}
