use crate::common::vector::Vector;

/// Inverse of a matrix
///
pub fn inv_normal(a: &[f64], n: usize) -> Vector<f64> {
    assert!(n * n == a.len(), "Matrix is not a square");
    let ainv: Vector<f64> = Vector::zeroed(n * n);

    ainv
}
