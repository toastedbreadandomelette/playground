#[cfg(test)]
mod test {
    use crate::common::vector::Vector;

    #[test]
    pub fn test_alloc() {
        let mut p: Vector<i32> = Vector::new(5);
        p[0] = 1;
        p[1] = 3;
        p[2] = 4;
        p[3] = 6;
        p[4] = 1 << 30;

        assert_eq!(p[0], 1);
        assert_eq!(p[1], 3);
        assert_eq!(p[2], 4);
        assert_eq!(p[3], 6);
        assert_eq!(p[4], 1 << 30);
    }

    #[test]
    pub fn test_iter() {
        let en = 65536;
        let p: Vector<usize> = (0..en).collect();

        assert!(p.iter().zip(0..65536).all(|(a, b)| *a == b));
    }

    #[test]
    pub fn filter_test() {
        let mut new_arr: Vector<u32> = (0..100).collect();
        let vec: Vector<u32> = (0..65536).collect();
        new_arr.extend_from_iter(&mut vec.iter());

        assert_eq!(new_arr.len(), 100 + 65536);
        // According to array alignment constraints, nearest big
        // value divisble by 32 is the below number.
        assert_eq!(new_arr.cap(), ((65536 + 100 + 31) >> 5) << 5);

        let another_arr: Vector<u32> = (0..1928).collect();
        new_arr.extend_from_iter(&mut another_arr.iter());
        assert_eq!(new_arr.len(), 100 + 1928 + 65536);
        // According to array alignment constraints, nearest big
        // value divisble by 32 is the below number.
        assert_eq!(new_arr.cap(), ((100 + 1928 + 65536 + 31) >> 5) << 5);

        assert!(new_arr
            .iter()
            .zip((0..100).chain(0..65536).chain(0..1928))
            .all(|(a, b)| *a == b));
    }

    #[test]
    pub fn test_alloc_2() {
        let p: Vector<usize> = Vector::from_range(12..24);

        assert!(p.iter().zip(12..24).all(|(a, b)| *a == b));
    }

    #[test]
    #[should_panic]
    pub fn test_out_of_bound() {
        let p: Vector<usize> = Vector::from_range(12..24);
        _ = p[14];
    }
}
