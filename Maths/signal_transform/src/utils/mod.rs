pub mod c64;
pub mod c64x2;
pub mod c64x4;
pub mod index_generator;

#[allow(unused)]
pub fn display_bin(vec: &[usize]) {
    print!("[");
    for (index, x) in vec.iter().enumerate() {
        print!("{x:08b}");
        if index < vec.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}

#[inline(always)]
pub fn close_to(o: f64, a: f64) -> bool {
    (o - a).abs() < 1e-4 + 1e-4 * o.abs()
}
