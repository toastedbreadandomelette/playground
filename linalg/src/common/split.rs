pub fn split_exact_4_mut<T>(
    a: &mut [T],
    n: usize,
) -> (&mut [T], &mut [T], &mut [T], &mut [T]) {
    assert_eq!(a.len() / n, 4);
    let aptr = a.as_mut_ptr();
    unsafe {
        (
            &mut *core::ptr::slice_from_raw_parts_mut(aptr, n),
            &mut *core::ptr::slice_from_raw_parts_mut(aptr.add(n), n),
            &mut *core::ptr::slice_from_raw_parts_mut(aptr.add(n << 1), n),
            &mut *core::ptr::slice_from_raw_parts_mut(aptr.add(n * 3), n),
        )
    }
}
