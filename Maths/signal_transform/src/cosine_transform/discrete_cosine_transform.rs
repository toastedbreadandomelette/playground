use crate::utils::c64::{Number, PI};
use core::ops::{Add, AddAssign};

pub fn dct1<T>(arr: &[T]) -> Vec<f64>
where
    T: From<T> + Copy + Number + AddAssign + Add,
    f64: From<T>,
{
    let x0: f64 = f64::from(arr[0]);
    let xn_1 = f64::from(*arr.last().unwrap());

    let (f, s) = ((x0 + xn_1) / 2.0, (x0 - xn_1) / 2.0);
    let sz = arr.len();
    let mut res: Vec<f64> = vec![0.0; sz];
    let base = PI / ((sz - 1) as f64);

    res.iter_mut().enumerate().for_each(|(k, elem)| {
        *elem = if (k & 1) == 1 { s } else { f };
        let incr = base * (k as f64);
        let mut base_angle: f64 = incr;
        *elem = arr.iter().take(sz - 1).skip(1).fold(0.0, |prev, curr| {
            let ans = prev + f64::from(*curr) * base_angle.cos();
            base_angle += incr;
            ans
        }) * 2.0;
    });

    res
}

pub fn dct2<T>(arr: &[T]) -> Vec<f64>
where
    T: From<T> + Copy + Number + AddAssign + Add,
    f64: From<T>,
{
    let sz = arr.len();
    let mut res: Vec<f64> = vec![0.0; sz];
    let base = PI / (sz as f64);

    res[0] = arr.iter().fold(0.0, |prev, curr| prev + f64::from(*curr));

    res.iter_mut().enumerate().skip(1).for_each(|(k, elem)| {
        let incr = base * (k as f64);
        let mut base_angle: f64 = incr * 0.5;
        *elem = arr.iter().fold(0.0, |prev, curr| {
            let ans = prev + f64::from(*curr) * base_angle.cos();
            base_angle += incr;
            ans
        }) * 2.0;
    });

    res
}
