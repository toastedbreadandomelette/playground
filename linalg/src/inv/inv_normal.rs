use vector::vector::Vector;

fn display_mat(arr: &[f64], n: usize) {
    arr.chunks_exact(n).for_each(|c| println!("{:?}", c));
    println!();
}

/// Inverse of a matrix
///
pub fn inv_normal(a: &[f64], n: usize) -> Vector<f64> {
    todo!()
    // assert!(n * n == a.len(), "Matrix is not a square");
    // let mut a_copy: Vector<f64> = a.iter().copied().collect();
    // let mut ainv: Vector<f64> = Vector::zeroed(a.len());
    // // println!("A");
    // // display_mat(&a_copy, n);
    // ainv.iter_mut().step_by(n + 1).for_each(|f| *f = 1.0);
    // for x in 0..n {
    //     // For each row, reduce rows at the bottom
    //     let start = x * (n + 1);
    //     // Starting from x + 1 row
    //     for row_next in (x + 1)..n {
    //         let factor = a_copy[start + (row_next - x) * n] / a_copy[start];
    //         a_copy[start + (row_next - x) * n] = 0.0;

    //         for col in (x + 1)..n {
    //             // println!("{}", start + (row_next - x) * n);
    //             a_copy[start + n * (row_next - x) + col - x] -=
    //                 a_copy[start + col - x] * factor;
    //         }

    //         for col in 0..n {
    //             ainv[n * row_next + col] -= ainv[n * x + col] * factor;
    //         }
    //     }
    //     // Reduce the current row
    //     let red_factor = 1.0 / a_copy[start];
    //     a_copy[start] = 1.0;
    //     for row in x + 1..n {
    //         a_copy[start + row - x] *= red_factor;
    //         ainv[start + row - x] *= red_factor;
    //     }

    //     for row in 0..x {
    //         ainv[x * n + row] *= red_factor;
    //     }
    // }

    // for r in (0..n).rev() {
    //     // let diag_index = r * (n + 1);
    //     // let elem = a_copy[diag_index];
    //     let inv_slice = &ainv[r * n..(r + 1) * n];
    //     // display_mat(&ainv, n);

    //     a_copy
    //         .chunks_exact_mut(n)
    //         .take(r)
    //         .zip(ainv.chunks_exact_mut(n).take(r))
    //         .for_each(|(ac, ai)| {
    //             let scale = ac[r];
    //             ac[r] -= scale;
    //             ai.iter_mut()
    //                 .zip(inv_slice.iter())
    //                 .for_each(|(ai_val, ac_val)| *ai_val -= scale * ac_val);
    //         });
    // }

    // ainv
}
