use std::marker::PhantomData;

#[inline(always)]
pub fn rev_chunk_exact_rem_slice_iter<'a, T>(
    arr: &'a [T],
    chunk_size: usize,
) -> RevChunkExactRemSliceIter<'a, T> {
    RevChunkExactRemSliceIter::new(arr, chunk_size)
}

#[inline(always)]
pub fn rev_chunk_exact_rem_slice_iter_mut<'a, T>(
    arr: &'a mut [T],
    chunk_size: usize,
) -> RevChunkExactRemSliceIterMut<'a, T> {
    RevChunkExactRemSliceIterMut::new(arr, chunk_size)
}
/// Returns an iterator that returns the chunk of an array, along with the remaining
/// slice of that array
pub struct RevChunkExactRemSliceIter<'a, T> {
    /// Slice of the array
    slice: &'a [T],
    /// Pointer to the split
    ptr: usize,
    /// Chunk size of that array
    chunk_size: usize,
}

impl<'a, T> RevChunkExactRemSliceIter<'a, T> {
    pub fn new(slice: &'a [T], chunk_size: usize) -> Self {
        let len = slice.len();
        assert!(
            len % chunk_size == 0,
            "Chunk size {chunk_size} is not a multiple of {len}"
        );
        Self {
            slice,
            ptr: 0,
            chunk_size,
        }
    }
}

impl<'a, T> Iterator for RevChunkExactRemSliceIter<'a, T> {
    type Item = (&'a [T], &'a [T]);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len() > self.ptr {
            let (first, last) = self
                .slice
                .split_at(self.slice.len() - self.chunk_size - self.ptr);
            self.ptr += self.chunk_size;
            self.slice = first;
            Some((first, last))
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_length = self.slice.len() / self.chunk_size;
        (remaining_length, Some(remaining_length))
    }
}

/// Returns an iterator that returns the chunk of an array, along with the remaining
/// slice of that array
pub struct RevChunkExactRemSliceIterMut<'a, T> {
    /// Slice of the array
    slice: *mut [T],
    /// Current pointer to be tracked.
    ptr: usize,
    /// Length of the slice
    len: usize,
    /// Chunk size of that array
    chunk_size: usize,
    /// Phantom Data
    _p: PhantomData<&'a T>,
}

impl<'a, T> RevChunkExactRemSliceIterMut<'a, T> {
    #[inline(always)]
    pub fn new(slice: &'a mut [T], chunk_size: usize) -> Self {
        let len = slice.len();
        assert!(
            len % chunk_size == 0,
            "Chunk size length {chunk_size} is not a multiple of array length {len}"
        );
        Self {
            slice: &mut slice[..],
            ptr: 0,
            len,
            chunk_size,
            _p: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RevChunkExactRemSliceIterMut<'a, T> {
    type Item = (&'a mut [T], &'a mut [T]);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.len {
            let curr_row = core::ptr::slice_from_raw_parts_mut(
                self.slice as *mut T,
                self.len - self.ptr - self.chunk_size,
            );

            self.ptr += self.chunk_size;
            let next_remaining = unsafe {
                core::ptr::slice_from_raw_parts_mut(
                    (self.slice as *mut T).add(self.len - self.ptr),
                    self.chunk_size,
                )
            };

            unsafe { Some((&mut *curr_row, &mut *next_remaining)) }
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_length = (self.len - self.ptr) / self.chunk_size;
        (remaining_length, Some(remaining_length))
    }
}
