use core::marker::PhantomData;

/// Iterator for diagonally iterating through the matrix
pub fn diagonal_iter<'a, T>(slice: &'a [T], len: usize) -> DiagonalIter<'a, T> {
    DiagonalIter::new(slice, len)
}

/// Iterator for diagonally iterating through the matrix
pub fn diagonal_iter_mut<'a, T>(
    slice: &'a mut [T],
    len: usize,
) -> DiagonalIterMut<'a, T> {
    DiagonalIterMut::new(slice, len)
}

/// Iterator for iterating the matrix diagonally
pub struct DiagonalIter<'a, T> {
    /// Slice of a 2-D Square array
    slice: &'a [T],
    /// Current pointer
    ptr: usize,
    /// Length of a 2-D Array.
    len: usize,
}

impl<'a, T> DiagonalIter<'a, T> {
    pub fn new(slice: &'a [T], len: usize) -> Self {
        Self { slice, len, ptr: 0 }
    }
}

impl<'a, T> Iterator for DiagonalIter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        (self.ptr + self.len + 1 < self.slice.len()).then(|| {
            let index = self.ptr;
            self.ptr += self.len + 1;
            &self.slice[index]
        })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let value = self.ptr / self.len;
        (value, Some(value))
    }
}

/// Iterator for iterating the matrix diagonally
pub struct DiagonalIterMut<'a, T> {
    /// Slice of a 2-D Square array
    slice: *mut T,
    /// Current pointer
    ptr: usize,
    /// Length of the slice
    slice_len: usize,
    /// Length of a 2-D Array.
    len: usize,
    /// Phantom data
    _phantom_data: PhantomData<&'a mut T>,
}

impl<'a, T> DiagonalIterMut<'a, T> {
    pub fn new(slice: &'a mut [T], len: usize) -> Self {
        let slice_len = slice.len();
        Self {
            slice: slice.as_mut_ptr(),
            slice_len,
            len,
            ptr: 0,
            _phantom_data: PhantomData,
        }
    }
}

impl<'a, T> Iterator for DiagonalIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.ptr < self.slice_len).then(|| {
            let index = self.ptr;
            self.ptr += self.len + 1;
            unsafe { &mut *self.slice.add(index) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let value = self.ptr / self.len;
        (value, Some(value))
    }
}
