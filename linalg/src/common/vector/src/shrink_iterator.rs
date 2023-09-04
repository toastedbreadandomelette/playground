use core::marker::PhantomData;

/// A iterator over a shrinking slice from head.
///
/// When slices are not evenly divisible, last element will yield remaining
/// size
#[derive(Debug)]
pub struct ShrinkHeadSlice<'a, T> {
    slice: &'a [T],
    slice_size: usize,
}

impl<'a, T> ShrinkHeadSlice<'a, T> {
    /// Creates a shrink head iterator
    #[inline]
    pub fn new(slice: &'a [T], slice_size: usize) -> Self {
        Self { slice, slice_size }
    }
}

impl<'a, T> Iterator for ShrinkHeadSlice<'a, T> {
    type Item = &'a [T];

    /// Advance to next value.
    ///
    /// This is a convenient implementation for
    /// shrinking
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let curr = &self.slice[..];

            self.slice = if self.slice_size <= self.slice.len() {
                let (_, next) = self.slice.split_at(self.slice_size);
                next
            } else {
                &[]
            };

            Some(curr)
        }
    }

    /// Get the size hint given the iterator
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() {
            (0, Some(0))
        } else {
            let total_slices =
                (self.slice.len() + self.slice_size - 1) / self.slice_size;
            (total_slices, Some(total_slices))
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.slice_size);

        if start >= self.slice.len() || overflow {
            self.slice = &[];
            None
        } else {
            let nth = &self.slice[start..];
            let (next, overflow) = start.overflowing_add(self.slice_size);
            self.slice = if next >= self.slice.len() || overflow {
                &[]
            } else {
                &self.slice[next..]
            };
            Some(nth)
        }
    }
}

/// A iterator over a mutable shrinking slice from head.
///
/// When slices are not evenly divisible, last element will yield remaining
/// size
#[derive(Debug)]
pub struct ShrinkHeadSliceMut<'a, T> {
    slice: *mut [T],
    len: usize,
    slice_size: usize,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T> ShrinkHeadSliceMut<'a, T> {
    /// Creates a shrink head iterator
    #[inline]
    pub fn new(slice: &'a mut [T], slice_size: usize) -> Self {
        Self {
            slice,
            slice_size,
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ShrinkHeadSliceMut<'a, T> {
    type Item = &'a mut [T];

    /// Advance to next value.
    ///
    /// This is a convenient implementation for
    /// shrinking
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            let min_length = core::cmp::min(self.len, self.slice_size);
            let current_mut_ptr = self.slice as *mut T;

            let next = core::ptr::slice_from_raw_parts_mut(
                unsafe { current_mut_ptr.add(self.slice_size).cast() },
                self.len - min_length,
            );

            self.slice = next;

            let current_shrinked_slice_start: *mut [T] =
                core::ptr::slice_from_raw_parts_mut(current_mut_ptr, self.len);

            self.len -= min_length;

            unsafe { Some(&mut *current_shrinked_slice_start) }
        }
    }

    /// Get the size hint given the iterator
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.len == 0 {
            (0, Some(0))
        } else {
            let total_slices =
                (self.len + self.slice_size - 1) / self.slice_size;
            (total_slices, Some(total_slices))
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.slice_size);

        if start >= self.len || overflow {
            (self.slice, self.len) = (&mut [], 0);
            None
        } else {
            // Manipulate the pointer
            // I hate my life
            let len = self.len;
            let (next_index, overflow) = start.overflowing_add(self.slice_size);

            let current_mut_ptr = unsafe { (self.slice as *mut T).add(start) };

            let next = if next_index > self.len || overflow {
                core::ptr::slice_from_raw_parts_mut(
                    unsafe { current_mut_ptr.add(next_index).cast() },
                    len - start,
                )
            } else {
                &mut []
            };

            (self.slice, self.len) = (next, self.len - start);

            let current_shrinked_slice_start: *mut [T] =
                core::ptr::slice_from_raw_parts_mut(current_mut_ptr, self.len);

            unsafe { Some(&mut *current_shrinked_slice_start) }
        }
    }
}
