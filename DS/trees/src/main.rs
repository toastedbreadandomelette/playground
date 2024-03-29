mod b_tree;
mod bstree;
mod common;

fn main() {
    let mut p = bstree::BSTree::<String>::new(None);
    let c = p.batch_insert(
        &(0..200).map(|c| c.to_string()).collect::<Vec<String>>(),
    );
    println!("{:?}", c);
}
