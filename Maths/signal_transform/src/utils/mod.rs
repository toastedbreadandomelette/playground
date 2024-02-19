pub mod c64;
pub mod c64x2;
pub mod index_generator;

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
