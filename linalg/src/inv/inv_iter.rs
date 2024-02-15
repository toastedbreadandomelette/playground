use crate::common::chunk_rem_slice_iter::chunk_exact_rem_slice_iter_mut;
use crate::common::diagonal_iter::diagonal_iter_mut;
use crate::common::rev_chunk_rem_slice_iter::rev_chunk_exact_rem_slice_iter_mut;
use vector::Vector;

pub fn inv_iter(a: &[f64], n: usize) -> Vector<f64> {
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
            acb.chunks_exact_mut(n)
                .zip(invb.chunks_exact_mut(n))
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
