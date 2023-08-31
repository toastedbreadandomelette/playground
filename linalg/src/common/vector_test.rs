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
    pub fn test_mutate_add_small_arr() {
        // Collect uses mutate_add: therefore, increases capacity 2 times
        // if exceeds, but base minimum capacity is `32`.
        let new_arr: Vector<u32> = (0..8).collect();
        assert_eq!(new_arr.len(), 8);
        assert_eq!(new_arr.cap(), 32);
    }

    #[test]
    pub fn test_mutate_add() {
        // Collect uses mutate_add: therefore, increases capacity 2 times
        // if exceeds, but base size is 32.
        let mut new_arr: Vector<u32> = (0..100).collect();
        assert_eq!(new_arr.len(), 100);
        assert_eq!(new_arr.cap(), 128);

        // This loop forces the addition to extend capacity to
        // Twice the original one.
        for x in 100..200 {
            new_arr.mutate_add(x);
        }

        assert!(new_arr.iter().zip(0..200).all(|(a, b)| *a == b));

        assert_eq!(new_arr.len(), 200);
        assert_eq!(new_arr.cap(), 256);
    }

    #[test]
    pub fn test_alloc_2() {
        let p: Vector<usize> = Vector::from_range(12..24);

        assert!(p.iter().zip(12..24).all(|(a, b)| *a == b));
    }

    #[test]
    fn test_for_loop() {
        let p: Vector<usize> = (0..100).step_by(2).collect();
        let mut start_value = 0;
        for x in &p {
            assert_eq!(start_value, *x);
            start_value += 2;
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bound() {
        let p: Vector<usize> = Vector::from_range(12..24);
        _ = p[14];
    }
}
