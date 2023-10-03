#[cfg(test)]
pub mod tests {
    use super::super::{shrink_iterator::ShrinkHeadSlice, vector::Vector};

    #[test]
    fn test_iter() {
        let slice: Vector<u32> = (0..11).collect();

        let mut iter = ShrinkHeadSlice::new(&slice, 2);

        assert_eq!(iter.next().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(iter.next().unwrap(), [2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(iter.next().unwrap(), [4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(iter.next().unwrap(), [6, 7, 8, 9, 10]);
        assert_eq!(iter.next().unwrap(), [8, 9, 10]);
        assert_eq!(iter.next().unwrap(), [10]);
        assert_eq!(iter.next(), None);

        let mut iter_2 = ShrinkHeadSlice::new(&slice, 3);

        assert_eq!(iter_2.next().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(iter_2.next().unwrap(), [3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(iter_2.next().unwrap(), [6, 7, 8, 9, 10]);
        assert_eq!(iter_2.next().unwrap(), [9, 10]);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_nth() {
        // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        let slice: Vector<u32> = (0..11).collect();

        let val = slice.shrink_head_mut(3).nth(3);
        assert!(*val.unwrap() == [9, 10]);

        let val = slice.shrink_head_mut(3).nth(2);
        assert!(*val.unwrap() == [6, 7, 8, 9, 10]);

        assert!(slice
            .shrink_head_mut(4)
            .nth(2)
            .is_some_and(|f| { f.len() == 3 && *f == [8, 9, 10] }));

        match slice.shrink_head_mut(4).nth(2) {
            Some(rem_slice) => *rem_slice.last_mut().unwrap() = 11,
            None => {}
        }

        assert!(*slice.last().unwrap() == 11);
    }

    #[test]
    fn test_skip() {
        let slice: Vector<u32> = (0..11).collect();
        let mut p = slice.shrink_head(2).skip(3);

        assert!(p.next().is_some_and(|val| { *val == [6, 7, 8, 9, 10] }));

        assert!(p.next().is_some_and(|val| { *val == [8, 9, 10] }));

        assert!(p.next().is_some_and(|val| { *val == [10] }));

        assert!(p.next().is_none());
    }

    #[test]
    fn test_iter_mut() {
        let slice: Vector<u32> = (0..11).collect();
        let mut shrinking_size = slice.len();

        slice.shrink_head_mut(2).for_each(|f| {
            assert_eq!(f.len(), shrinking_size);
            f[0] += 10;
            shrinking_size -= if shrinking_size < 2 {
                shrinking_size
            } else {
                2
            };
        });

        let mut res: Vector<u32> = (0..11).collect();
        res.iter_mut().step_by(2).for_each(|f| *f += 10);
        assert!(slice == res);
    }

    #[test]
    fn some_new_test() {
        let slice: Vector<usize> = (0..1 << 7).collect();
        let mut index = 1 << 7;

        slice.shrink_head_mut(8).enumerate().for_each(|(st, p)| {
            assert_eq!(index, p.len());
            let start = st * 8;

            let min = core::cmp::min(p.len(), 8);
            let slice = (start..start + min).collect::<Vector<usize>>();
            assert_eq!(&p[0..min], &slice[..]);

            // Change values of first index by 3, third
            // index by 8, 5th index by 7
            p[0] += 3;
            p[3] += 8;
            p[5] += 7;
            p[7] += 12;

            index -= 8;
        });

        slice.chunks_exact(8).enumerate().for_each(|(idx, c)| {
            let i = idx * 8;

            assert_eq!(c[0], i + 3);
            assert_eq!(c[3], i + 3 + 8);
            assert_eq!(c[5], i + 5 + 7);
            assert_eq!(c[7], i + 7 + 12);
        });
    }
}
