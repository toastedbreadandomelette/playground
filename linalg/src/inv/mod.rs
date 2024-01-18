pub mod inv_normal;
use crate::common::close_to;
use crate::matmul::matmul_normal;
use rand::Rng;
use vector::Vector;

pub fn check_inverse(a: &[f64], n: usize) -> bool {
    a.iter().enumerate().all(|(i, el)| {
        if i % (n + 1) == 0 {
            close_to(*el, 1.0)
        } else {
            close_to(*el, 0.0)
        }
    })
}

pub fn inverse() {
    let sz = 1024;
    let mut rng = rand::thread_rng();
    let a: Vector<f64> = (0..sz * sz)
        .map(|_| ((rng.gen::<f64>().abs() + (sz as f64)) * (sz as f64)).floor())
        .collect();

    let mut t = std::time::Instant::now();
    let orig = inv_normal::inv_normal(&a, sz);
    println!("Naive: {}ms", t.elapsed().as_millis());

    let check = matmul_normal::matmul_normal(&a, &orig, (sz, sz), (sz, sz));
    println!("{}", check_inverse(&check, sz));
}
