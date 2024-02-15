use crate::common::diagonal_iter::diagonal_iter_mut;
use vector::Vector;

pub fn display_mat(arr: &[f64], n: usize) {
    arr.chunks_exact(n).for_each(|c| println!("{:?}", c));
    println!();
}

pub fn check_and_swap(a: &mut [f64], inv: &mut [f64], n: usize) -> bool {
    for x in (0..n * n).step_by(n + 1) {
        if a[x] == 0.0 {
            let (row, col) = (x / n, x % n);
            let mut found = false;
            for next_x in row + 1..n {
                if a[next_x * n + col] != 0.0 {
                    found = true;
                    for col in 0..n {
                        (a[row * n + col], a[next_x * n + col]) =
                            (a[next_x * n + col], a[row * n + col]);
                        (inv[row * n + col], inv[next_x * n + col]) =
                            (inv[next_x * n + col], inv[row * n + col]);
                    }
                }
            }

            if !found {
                return false;
            }
        }
    }

    true
}

/// Inverse of a matrix
pub fn inv_normal(a: &[f64], n: usize) -> Vector<f64> {
    assert_eq!(a.len(), n * n);

    let mut ac: Vector<f64> = Vector::zeroed(n * n);
    ac.iter_mut().zip(a).for_each(|(aci, ai)| *aci = *ai);
    let mut inv = Vector::zeroed(n * n);

    diagonal_iter_mut(&mut inv, n).for_each(|c| *c = 1.0);

    if !check_and_swap(&mut ac, &mut inv, n) {
        panic!("No inverse for this matrix");
    }

    // For each row
    for row in 0..n - 1 {
        // Make the a[i][i] as 1, by first reducing all values beneath that value to zero
        for next_row in row + 1..n {
            let value = ac[next_row * n + row] / ac[row * n + row];
            for col in 0..n {
                ac[next_row * n + col] -= value * ac[row * n + col];
                inv[next_row * n + col] -= value * inv[row * n + col];
            }

            ac[next_row * n + row] = 0.0;
        }

        let value = ac[row * n + row];
        for col in 0..n {
            ac[row * n + col] /= value;
            inv[row * n + col] /= value;
        }
    }

    let last_row = n - 1;
    let value = *ac.last().unwrap();

    for col in 0..n {
        inv[last_row * n + col] /= value;
    }
    *ac.last_mut().unwrap() = 1.0;

    for row in (1..n).rev() {
        for prev_row in (0..row).rev() {
            let value = ac[prev_row * n + row];
            for col in 0..n {
                inv[prev_row * n + col] -= value * inv[row * n + col];
            }

            ac[prev_row * n + row] = 0.0;
        }
    }

    inv
}
