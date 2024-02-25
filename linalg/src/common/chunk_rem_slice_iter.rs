use std::marker::PhantomData;

#[allow(unused)]
pub fn chunk_exact_rem_slice_iter<T>(
    arr: &[T],
    chunk_size: usize,
) -> ChunkExactRemSliceIter<'_, T> {
    ChunkExactRemSliceIter::new(arr, chunk_size)
}

pub fn chunk_exact_rem_slice_iter_mut<T>(
    arr: &mut [T],
    chunk_size: usize,
) -> ChunkExactRemSliceIterMut<'_, T> {
    ChunkExactRemSliceIterMut::new(arr, chunk_size)
}

/// Returns an iterator that returns the chunk of an array, along with the remaining
/// slice of that array
pub struct ChunkExactRemSliceIter<'a, T> {
    /// Slice of the array
    slice: &'a [T],
    /// Chunk size of that array
    chunk_size: usize,
}

impl<'a, T> ChunkExactRemSliceIter<'a, T> {
    #[allow(unused)]
    pub fn new(slice: &'a [T], chunk_size: usize) -> Self {
        let len = slice.len();
        assert!(
            len % chunk_size == 0,
            "Chunk size {chunk_size} is not a multiple of {len}"
        );
        Self { slice, chunk_size }
    }
}

impl<'a, T> Iterator for ChunkExactRemSliceIter<'a, T> {
    type Item = (&'a [T], &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        (self.slice.len() > 0).then(|| {
            let (first, last) = self.slice.split_at(self.chunk_size);
            self.slice = last;
            (first, last)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_length = self.slice.len() / self.chunk_size;
        (remaining_length, Some(remaining_length))
    }
}

/// Returns an iterator that returns the chunk of an array, along with the remaining
/// slice of that array
pub struct ChunkExactRemSliceIterMut<'a, T> {
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

impl<'a, T> ChunkExactRemSliceIterMut<'a, T> {
    pub fn new(slice: &'a mut [T], chunk_size: usize) -> Self {
        let len = slice.len();
        assert!(
            len % chunk_size == 0,
            "Chunk size {chunk_size} is not a multiple of {len}"
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

impl<'a, T> Iterator for ChunkExactRemSliceIterMut<'a, T> {
    type Item = (&'a mut [T], &'a mut [T]);

    fn next(&mut self) -> Option<Self::Item> {
        (self.ptr < self.len).then(|| {
            let curr_row = unsafe {
                core::ptr::slice_from_raw_parts_mut(
                    (self.slice as *mut T).add(self.ptr),
                    self.chunk_size,
                )
            };
            self.ptr += self.chunk_size;
            let next_remaining = unsafe {
                core::ptr::slice_from_raw_parts_mut(
                    (self.slice as *mut T).add(self.ptr),
                    self.len - self.ptr,
                )
            };
            unsafe { (&mut *curr_row, &mut *next_remaining) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_length = self.len / self.chunk_size;
        (remaining_length, Some(remaining_length))
    }
}
