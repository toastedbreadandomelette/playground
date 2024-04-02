use crate::common::chunk_rem_slice_iter::chunk_exact_rem_slice_iter_mut;
use crate::common::diagonal_iter::diagonal_iter_mut;
use crate::common::rev_chunk_rem_slice_iter::rev_chunk_exact_rem_slice_iter_mut;
use crate::common::split::{split_exact_3_mut, split_exact_4_mut};
use crate::inv::{div, div2, div3, div4};
use vector::Vector;

pub fn inv_iter_simd(a: &[f64], n: usize) -> Vector<f64> {
    let mut ac: Vector<f64> = Vector::zeroed(n * n);
    ac.iter_mut().zip(a).for_each(|(aci, ai)| *aci = *ai);

    let mut inv: Vector<f64> = Vector::zeroed(n * n);
    diagonal_iter_mut(&mut inv, n).for_each(|el| *el = 1.0);

    if !crate::inv::inv_normal::check_and_swap(&mut ac, &mut inv, n) {
        panic!("No inverse for this matrix.");
    }

    chunk_exact_rem_slice_iter_mut(&mut ac, n)
        .zip(chunk_exact_rem_slice_iter_mut(&mut inv, n))
        .enumerate()
        .for_each(|(row, ((acr, acb), (invr, invb)))| {
            acb.chunks_exact_mut(4 * n)
                .zip(invb.chunks_exact_mut(4 * n))
                .enumerate()
                .for_each(|(_, (acbr, invbr))| {
                    let (value0, value1, value2, value3) = (
                        acbr[row] / acr[row],
                        acbr[row + n] / acr[row],
                        acbr[row + n + n] / acr[row],
                        acbr[row + n + n + n] / acr[row],
                    );

                    let (acbr0, acbr1, acbr2, acbr3) =
                        split_exact_4_mut(acbr, n);
                    let (invbr0, invbr1, invbr2, invbr3) =
                        split_exact_4_mut(invbr, n);

                    unsafe {
                        div4(
                            acbr0, invbr0, acbr1, invbr1, acbr2, invbr2, acbr3,
                            invbr3, acr, invr, value0, value1, value2, value3,
                        )
                    };

                    (
                        acbr[row],
                        acbr[row + n],
                        acbr[row + n + n],
                        acbr[row + n + n + n],
                    ) = (0.0, 0.0, 0.0, 0.0);
                });

            let acbrem = acb.chunks_exact_mut(4 * n).into_remainder();
            let invbrem = invb.chunks_exact_mut(4 * n).into_remainder();
            let remstep = acbrem.len() / n;

            match remstep {
                1 => {
                    let value0 = acbrem[row] / acr[row];
                    unsafe {
                        div(acbrem, invbrem, acr, invr, value0);
                    }
                    acbrem[row] = 0.0;
                }
                2 => {
                    let value0 = acbrem[row] / acr[row];
                    let value1 = acbrem[row + n] / acr[row];

                    let (acbr0, acbr1) = acbrem.split_at_mut(n);
                    let (invbr0, invbr1) = invbrem.split_at_mut(n);
                    unsafe {
                        div2(
                            acbr0, invbr0, acbr1, invbr1, acr, invr, value0,
                            value1,
                        );
                    }
                    acbrem[row] = 0.0;
                    acbrem[row + n] = 0.0;
                }
                3 => {
                    let value0 = acbrem[row] / acr[row];
                    let value1 = acbrem[row + n] / acr[row];
                    let value2 = acbrem[row + n + n] / acr[row];

                    let (acbr0, acbr1, acbr2) = split_exact_3_mut(acbrem, n);
                    let (invbr0, invbr1, invbr2) =
                        split_exact_3_mut(invbrem, n);
                    unsafe {
                        div3(
                            acbr0, invbr0, acbr1, invbr1, acbr2, invbr2, acr,
                            invr, value0, value1, value2,
                        );
                    }
                    acbrem[row] = 0.0;
                    acbrem[row + n] = 0.0;
                    acbrem[row + n + n] = 0.0;
                }
                _ => {}
            }

            acbrem
                .chunks_exact_mut(n)
                .zip(invbrem.chunks_exact_mut(n))
                .enumerate()
                .filter(|(_, (acbr, _))| acbr[row] != 0.0)
                .for_each(|(_, (acbr, invbr))| {
                    let value = acbr[row] / acr[row];
                    acbr.iter_mut()
                        .zip(invbr.iter_mut())
                        .zip(acr.iter())
                        .zip(invr.iter())
                        .skip(row)
                        .for_each(|(((acbre, invre), acre), inve)| {
                            *acbre -= value * acre;
                            *invre -= value * inve;
                        });

                    invbr.iter_mut().zip(invr.iter()).take(row).for_each(
                        |(invbre, invre)| {
                            *invbre -= value * invre;
                        },
                    );

                    acbr[row] = 0.0;
                });

            // Use inverse value for multplication instead of division
            let value = 1.0 / acr[row];
            acr.iter_mut()
                .zip(invr.iter_mut())
                .for_each(|(acre, invre)| {
                    *acre *= value;
                    *invre *= value;
                })
        });

    rev_chunk_exact_rem_slice_iter_mut(&mut ac, n)
        .zip(rev_chunk_exact_rem_slice_iter_mut(&mut inv, n))
        .enumerate()
        .for_each(|(row, ((aca, _), (inva, invr)))| {
            let index = n - 1 - row;

            aca.chunks_exact_mut(n)
                .zip(inva.chunks_exact_mut(n))
                .enumerate()
                .for_each(|(_, (acar, invar))| {
                    let value = acar[index];
                    invar
                        .iter_mut()
                        .zip(invr.iter())
                        .for_each(|(invare, invre)| *invare -= value * invre);

                    acar[index] = 0.0;
                });
        });

    inv
}
