use crate::utils::c64::{Number, C64, PI};
use core::ops::{Add, AddAssign, Mul};
use vector::Vector;

/// Faster DFT: Use multiple accumulator
pub fn dft_fast<T>(arr: &[T]) -> Vector<C64>
where
    T: Number + AddAssign + Mul + Add + core::convert::Into<f64> + Copy,
    f64: core::convert::From<T>,
{
    let len = arr.len();
    let mut res = Vector::zeroed(arr.len());
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::unit_ag_conj(ag);
    let wlen2 = wlen * wlen;
    let wlen3 = wlen2 * wlen;
    let wlen4 = wlen3 * wlen;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen3;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) =
            (C64::unit(), C64::unit(), C64::unit(), C64::unit());
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

        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;

        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;

        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;

        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::unit();

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res
}

pub fn dft_fast_c64(arr: &[C64]) -> Vector<C64> {
    let len = arr.len();
    let mut res = Vector::zeroed(arr.len());
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::unit_ag_conj(ag);
    let wlen2 = wlen * wlen;
    let wlen3 = wlen2 * wlen;
    let wlen4 = wlen3 * wlen;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen3;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) =
            (C64::unit(), C64::unit(), C64::unit(), C64::unit());
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

        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;

        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;

        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;

        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::unit();

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res
}

/// Faster DFT: Use multiple accumulator
pub fn idft_fast<T>(arr: &[C64]) -> Vector<T>
where
    T: From<T> + AddAssign + Mul + Copy + Number + core::convert::From<f64>,
    f64: core::convert::Into<T>,
{
    let len = arr.len();
    let mut res = Vector::zeroed(arr.len());
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::unit_ag(ag);
    let wlen2 = wlen * wlen;
    let wlen3 = wlen2 * wlen;
    let wlen4 = wlen3 * wlen;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen2 * wlen;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) =
            (C64::unit(), C64::unit(), C64::unit(), C64::unit());
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

        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;

        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;

        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;

        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::unit();

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res.iter().map(|x| (x.real / len as f64).into()).collect()
}

pub fn idft_fast_c64(arr: &[C64]) -> Vector<C64> {
    let len = arr.len();
    let mut res = Vector::zeroed(arr.len());
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::unit_ag(ag);
    let wlen2 = wlen * wlen;
    let wlen3 = wlen2 * wlen;
    let wlen4 = wlen3 * wlen;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen2 * wlen;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) =
            (C64::unit(), C64::unit(), C64::unit(), C64::unit());
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

        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;

        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;

        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;

        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::unit();

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res
}

#[test]
pub fn test_fast_dft_idft() {
    let sz = 8192;
    let inp: Vector<f64> = (0..sz).map(|x| x as f64).collect();
    let val = dft_fast::<f64>(&inp);
    let orig: Vector<f64> = idft_fast::<f64>(&val);

    assert!(orig
        .iter()
        .zip(inp.iter())
        .all(|(elem, inp)| { (*elem - inp).abs() < 1e-4 + 1e-4 * elem.abs() }));
}

pub fn dft_fast_c64_simd(arr: &[C64]) -> Vector<C64> {
    let len = arr.len();
    let mut res = Vector::zeroed(arr.len());
    let ag = 2.0 * PI / (len as f64);

    let wlen = C64::unit_ag_conj(ag);
    let wlen2 = wlen * wlen;
    let wlen3 = wlen2 * wlen;
    let wlen4 = wlen3 * wlen;

    let mut wstart0 = wlen;
    let mut wstart1 = wlen2;
    let mut wstart2 = wlen3;
    let mut wstart3 = wlen4;

    res[0] = arr.iter().fold(C64::zero(), |p, c| p + *c);

    res[1..].chunks_exact_mut(4).for_each(|elem| {
        let (mut w0, mut w1, mut w2, mut w3) =
            (C64::unit(), C64::unit(), C64::unit(), C64::unit());
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

        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;
        wstart0 *= wlen;

        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;
        wstart1 *= wlen;

        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;
        wstart2 *= wlen;

        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
        wstart3 *= wlen;
    });

    let rem = res[1..].chunks_exact_mut(4).into_remainder();

    rem.iter_mut().for_each(|res| {
        let mut w = C64::unit();

        arr.iter().for_each(|y| {
            *res += w * *y;
            w *= wstart0;
        });

        wstart0 *= wlen;
    });

    res
}

// void dft_subarray_inplace(c64 *array, i32 start, i32 end) {
//     const usize n = end - start;
//     c64 dest[n];
//     c64x2 accumulator = C64x2::zero();
//
//     i32 rem = (end - start) & 7;
//
//     for (usize i = start; i < (end - rem); i += 8) {
//         f64x4 acc = C64x2::fromptr(array + i);
//         accumulator = C64x2::add(accumulator, acc);
//
//         acc = C64x2::fromptr(array + i + 2);
//         accumulator = C64x2::add(accumulator, acc);
//
//         acc = C64x2::fromptr(array + i + 4);
//         accumulator = C64x2::add(accumulator, acc);
//
//         acc = C64x2::fromptr(array + i + 6);
//         accumulator = C64x2::add(accumulator, acc);
//     }
//
//     for (i32 i = (end - rem); i < end; ++i) {
//         dest[0] += array[i];
//     }
//
//     dest[0] += C64x2::reduce_sum(accumulator);
//
//     f64 angle = Math::pi_2 / n;
//     c64 wlen = c64(::cos(angle), -::sin(angle));
//     c64 wlen2 = wlen * wlen;
//     c64 wlen4 = wlen2 * wlen2;
//
//     c64 wstart = wlen;
//     rem = (end - start) & 7;
//     c64 buffer[2];
//     i32 nrem = (n - 1) & 3;
//     c64 wstart1, wstart2, wstart3, wstart4;
//     constexpr c64 unit = c64(1, 0);
//     for (i32 index = 1; index < (n - nrem); index += 4) {
//         wstart1 = wstart;
//         wstart2 = wstart1 * wlen;
//         wstart3 = wstart2 * wlen;
//         wstart4 = wstart3 * wlen;
//
//         c64x2 ws0 = C64x2::set(unit, wstart1);
//         c64x2 ws1 = C64x2::set(unit, wstart2);
//         c64x2 ws2 = C64x2::set(unit, wstart3);
//         c64x2 ws3 = C64x2::set(unit, wstart4);
//
//         c64x2 wsstart0 = C64x2::uni(wstart1 * wstart1);
//         c64x2 wsstart1 = C64x2::uni(wstart2 * wstart2);
//         c64x2 wsstart2 = C64x2::uni(wstart3 * wstart3);
//         c64x2 wsstart3 = C64x2::uni(wstart4 * wstart4);
//
//         c64x2 accumulator0 = C64x2::zero();
//         c64x2 accumulator1 = C64x2::zero();
//         c64x2 accumulator2 = C64x2::zero();
//         c64x2 accumulator3 = C64x2::zero();
//
//         for (i32 i = start; i < (end - rem); i += 8) {
//             c64x2 acc = C64x2::set(array[i], array[i + 1]);
//             accumulator0 = C64x2::add(accumulator0, C64x2::mul(acc, ws0));
//             accumulator1 = C64x2::add(accumulator1, C64x2::mul(acc, ws1));
//             accumulator2 = C64x2::add(accumulator2, C64x2::mul(acc, ws2));
//             accumulator3 = C64x2::add(accumulator3, C64x2::mul(acc, ws3));
//             ws0 = C64x2::mul(ws0, wsstart0);
//             ws1 = C64x2::mul(ws1, wsstart1);
//             ws2 = C64x2::mul(ws2, wsstart2);
//             ws3 = C64x2::mul(ws3, wsstart3);
//
//             acc = C64x2::set(array[i + 2], array[i + 3]);
//             accumulator0 = C64x2::add(accumulator0, C64x2::mul(acc, ws0));
//             accumulator1 = C64x2::add(accumulator1, C64x2::mul(acc, ws1));
//             accumulator2 = C64x2::add(accumulator2, C64x2::mul(acc, ws2));
//             accumulator3 = C64x2::add(accumulator3, C64x2::mul(acc, ws3));
//             ws0 = C64x2::mul(ws0, wsstart0);
//             ws1 = C64x2::mul(ws1, wsstart1);
//             ws2 = C64x2::mul(ws2, wsstart2);
//             ws3 = C64x2::mul(ws3, wsstart3);
//
//             acc = C64x2::set(array[i + 4], array[i + 5]);
//             accumulator0 = C64x2::add(accumulator0, C64x2::mul(acc, ws0));
//             accumulator1 = C64x2::add(accumulator1, C64x2::mul(acc, ws1));
//             accumulator2 = C64x2::add(accumulator2, C64x2::mul(acc, ws2));
//             accumulator3 = C64x2::add(accumulator3, C64x2::mul(acc, ws3));
//             ws0 = C64x2::mul(ws0, wsstart0);
//             ws1 = C64x2::mul(ws1, wsstart1);
//             ws2 = C64x2::mul(ws2, wsstart2);
//             ws3 = C64x2::mul(ws3, wsstart3);
//
//             acc = C64x2::set(array[i + 6], array[i + 7]);
//             accumulator0 = C64x2::add(accumulator0, C64x2::mul(acc, ws0));
//             accumulator1 = C64x2::add(accumulator1, C64x2::mul(acc, ws1));
//             accumulator2 = C64x2::add(accumulator2, C64x2::mul(acc, ws2));
//             accumulator3 = C64x2::add(accumulator3, C64x2::mul(acc, ws3));
//             ws0 = C64x2::mul(ws0, wsstart0);
//             ws1 = C64x2::mul(ws1, wsstart1);
//             ws2 = C64x2::mul(ws2, wsstart2);
//             ws3 = C64x2::mul(ws3, wsstart3);
//         }
//         C64x2::storeptr(buffer, ws0);
//         c64 w0 = buffer[0];
//         C64x2::storeptr(buffer, ws1);
//         c64 w1 = buffer[0];
//         C64x2::storeptr(buffer, ws2);
//         c64 w2 = buffer[0];
//         C64x2::storeptr(buffer, ws3);
//         c64 w3 = buffer[0];
//
//         for (i32 i = (end - rem); i < end; ++i) {
//             c64 a = array[i];
//             dest[index] += a * w0;
//             w0 *= wstart1;
//             dest[index + 1] += a * w1;
//             w1 *= wstart2;
//             dest[index + 2] += a * w2;
//             w2 *= wstart3;
//             dest[index + 3] += a * w3;
//             w3 *= wstart4;
//         }
//
//         wstart *= wlen4;
//
//         dest[index] += C64x2::reduce_sum(accumulator0);
//         dest[index + 1] += C64x2::reduce_sum(accumulator1);
//         dest[index + 2] += C64x2::reduce_sum(accumulator2);
//         dest[index + 3] += C64x2::reduce_sum(accumulator3);
//     }
//
//     for (i32 index = (n - nrem); index < n; ++index) {
//         c64 w = {1, 0};
//         for (usize i = start; i < end; ++i) {
//             dest[index] += array[i] * w;
//             w *= wstart;
//         }
//         wstart *= wlen;
//     }
//
//     for (i32 i = start; i < end; ++i) {
//         array[i] = dest[i - start];
//     }
//
//     // aligned_free(dest);
// }
