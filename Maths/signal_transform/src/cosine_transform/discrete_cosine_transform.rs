use std::ops::{Add, AddAssign};
use core::simd::f64x4;
use crate::utils::complex::{Number, PI};

pub fn dct1<T>(arr: &Vec<T>) -> Vec<f64> where T: From<T> + Copy + Number + AddAssign + Add, f64: From<T> {
    let x0: f64 = f64::from(arr[0]);
    let xn_1 = f64::from(*arr.last().unwrap());

    let f = (x0 + xn_1) / 2.0;
    let s = (x0 - xn_1) / 2.0;
    let sz = arr.len();

    let mut res: Vec<f64> = vec![0.0; sz];

    let base = PI / ((sz - 1) as f64);

    for k in 0..sz {
        res[k] = if (k & 1) == 1 { s } else { f };
        let incr = base * (k as f64);
        let mut base_angle: f64 = incr;
        for n in 1..sz-1 {
            res[k] += f64::from(arr[n]) * base_angle.cos();
            base_angle += incr;
        }
        res[k] *= 2.0;
    }

    res
}

pub fn dct2<T>(arr: &Vec<T>) -> Vec<f64> where T: From<T> + Copy + Number + AddAssign + Add, f64: From<T> {
    let sz = arr.len();
    let mut res: Vec<f64> = vec![0.0; sz];
    let base = PI / (sz as f64);

    res[0] = arr.iter().fold(0.0, |prev, curr| prev + f64::from(*curr));

    for k in 1..sz {
        let incr = base * (k as f64);
        let mut base_angle: f64 = incr * 0.5;
        let mut ans = 0.0;
        for n in 0..sz {
            ans += f64::from(arr[n]) * base_angle.cos();
            base_angle += incr;
        }
        res[k] = ans * 2.0;
    }

    res
}
