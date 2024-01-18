use vector::Vector;

pub fn display_mat(arr: &[f64], n: usize) {
    arr.chunks_exact(n).for_each(|c| println!("{:?}", c));
    println!();
}

fn swap_rows(a: &mut [f64], b: &mut [f64]) {
    assert_eq!(a.len(), b.len());
    a.iter_mut()
        .zip(b.iter_mut())
        .for_each(|(a, b)| (*a, *b) = (*b, *a));
}

/// Inverse of a matrix
pub fn inv_normal(a: &[f64], n: usize) -> Vector<f64> {
    assert_eq!(a.len(), n * n);

    let mut ac = a.to_vec();
    let mut inv = (0..n * n)
        .map(|c| if c % (n + 1) == 0 { 1.0 } else { 0.0 })
        .collect::<Vector<f64>>();

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
                // ac[prev_row * n + col] -= value * ac[row * n + col];
                inv[prev_row * n + col] -= value * inv[row * n + col];
            }

            ac[prev_row * n + row] = 0.0;
        }
    }

    inv
}
