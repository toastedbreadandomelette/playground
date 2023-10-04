use core::ops::{Add, AddAssign, Mul};

use crate::utils::c64::{Number, C64, PI};

/// Faster DFT: Use multiple accumulator
pub fn dft_fast<T>(arr: &[T]) -> Vec<C64>
where
    T: Number + AddAssign + Mul + Add + core::convert::Into<f64> + Copy,
    f64: core::convert::From<T>,
{
    let len = arr.len();
    let mut res = vec![C64::zero(); arr.len()];
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::new(ag.cos(), -ag.sin());
    let wlen2 = wlen * wlen;
    let wlen4 = wlen2 * wlen2;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen2 * wlen;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) = (
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
        );
        let (mut acc0, mut acc1, mut acc2, mut acc3) =
            (C64::zero(), C64::zero(), C64::zero(), C64::zero());

        arr.iter().for_each(|val| {
            acc0 += w0 * *val;
            acc1 += w1 * *val;
            acc2 += w2 * *val;
            acc3 += w3 * *val;

            w0 *= wstart0;
            w1 *= wstart1;
            w2 *= wstart2;
            w3 *= wstart3;
        });

        (elem[0], elem[1], elem[2], elem[3]) = (acc0, acc1, acc2, acc3);

        wstart0 *= wlen4;
        wstart1 *= wlen4;
        wstart2 *= wlen4;
        wstart3 *= wlen4;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::new(1.0, 0.0);

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res
}

/// Faster DFT: Use multiple accumulator
pub fn idft_fast<T>(arr: &[C64]) -> Vec<T>
where
    T: From<T> + AddAssign + Mul + Copy + Number + core::convert::From<f64>,
    f64: core::convert::Into<T>,
{
    let len = arr.len();
    let mut res = vec![C64::zero(); arr.len()];
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::new(ag.cos(), ag.sin());
    let wlen2 = wlen * wlen;
    let wlen4 = wlen2 * wlen2;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen2 * wlen;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) = (
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
            C64::new(1.0, 0.0),
        );
        let (mut acc0, mut acc1, mut acc2, mut acc3) =
            (C64::zero(), C64::zero(), C64::zero(), C64::zero());

        arr.iter().for_each(|val| {
            acc0 += w0 * *val;
            acc1 += w1 * *val;
            acc2 += w2 * *val;
            acc3 += w3 * *val;

            w0 *= wstart0;
            w1 *= wstart1;
            w2 *= wstart2;
            w3 *= wstart3;
        });

        (elem[0], elem[1], elem[2], elem[3]) = (acc0, acc1, acc2, acc3);

        wstart0 *= wlen4;
        wstart1 *= wlen4;
        wstart2 *= wlen4;
        wstart3 *= wlen4;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::new(1.0, 0.0);

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res.iter()
        .map(|x| ((*x) / (len as f64)).real.into())
        .collect::<Vec<T>>()
}

#[test]
pub fn test_fast_dft_idft() {
    let sz = 8192;
    let inp: Vec<f64> = (0..sz).map(|x| x as f64).collect();
    let val = dft_fast::<f64>(&inp);
    let orig: Vec<f64> = idft_fast::<f64>(&val);

    assert!(orig
        .iter()
        .zip(inp.iter())
        .all(|(elem, inp)| { (*elem - inp).abs() < 1e-4 + 1e-4 * elem.abs() }));
}
