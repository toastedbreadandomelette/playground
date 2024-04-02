pub mod matmul_modulo;
use rand::Rng;
use vector::Vector;

use self::matmul_modulo::matmul_modulo;

pub fn matmul_mod() {
    let sz: usize = 1024;
    let mut rng = rand::thread_rng();
    let a: Vector<u64> = (0..sz * sz)
        .map(|_| rng.gen::<u64>())
        .collect();
    let b: Vector<u64> = (0..sz * sz)
        .map(|_| rng.gen::<u64>())
        .collect();

    let t = std::time::Instant::now();
    let _ = matmul_modulo(&a, &b, (sz, sz), (sz, sz), 100);
    println!("Elapsed time: {}ms", t.elapsed().as_millis());
}