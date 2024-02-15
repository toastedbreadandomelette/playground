#[cfg(test)]
pub mod tests {
    use crate::common::chunk_rem_slice_iter::chunk_exact_rem_slice_iter;
    use crate::common::rev_chunk_rem_slice_iter::rev_chunk_exact_rem_slice_iter_mut;
    use vector::Vector;

    #[test]
    pub fn test_chunk_rem_slice() {
        let arr = (0..10).collect::<Vector<usize>>();

        chunk_exact_rem_slice_iter(&arr, 2)
            .enumerate()
            .for_each(|(i, s)| {
                let index = i * 2;
                let value: (Vector<usize>, Vector<usize>) =
                    ((index..index + 2).collect(), (index + 2..10).collect());
                assert_eq!(s, (&value.0[..], &value.1[..]));
            });
    }

    #[test]
    pub fn test_rev_chunk_rem_slice() {
        let mut arr = (0..10).collect::<Vector<usize>>();

        rev_chunk_exact_rem_slice_iter_mut(&mut arr, 2)
            .enumerate()
            .for_each(|(i, s)| {
                let index = 10 - (i + 1) * 2;
                let mut value: (Vector<usize>, Vector<usize>) =
                    ((0..index).collect(), (index..index + 2).collect());
                println!("{s:?}");
                assert_eq!(s, (&mut value.0[..], &mut value.1[..]));
            });
    }
}
