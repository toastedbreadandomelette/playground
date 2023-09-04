use core::marker::PhantomData;

pub struct ChunksWithPrePostSlice<'a, T> {
    /// Prefix slice of a given slice
    pre_slice: &'a [T],
    /// Current window slice
    current_working_slice: &'a [T],
    /// Post slice
    post_slice: &'a [T],
    /// Window size of [`current_working_slice`]
    window_size: usize,
}

impl<'a, T> ChunksWithPrePostSlice<'a, T> {
    /// Chunk, except you're provided with passed values,
    /// and next slice as well.
    #[inline(always)]
    pub fn new(slice: &'a [T], window_size: usize) -> Self {
        // Clamp the window_size first
        let min_size = core::cmp::min(window_size, slice.len());
        Self {
            pre_slice: &[],
            current_working_slice: &slice[..min_size],
            post_slice: &slice[min_size..],
            window_size,
        }
    }
}

impl<'a, T> Iterator for ChunksWithPrePostSlice<'a, T> {
    type Item = (&'a [T], &'a [T], &'a [T]);

    /// Get the next slice, with complete pre-slice and post-slice
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_working_slice.is_empty() {
            None
        } else {
            let (pre_slice, current_slice, post_slice) =
                (self.pre_slice, self.current_working_slice, self.post_slice);

            let clamped_next_post_slice_len =
                core::cmp::min(self.window_size, self.post_slice.len());
            let next_pre_slice_len =
                pre_slice.len() + clamped_next_post_slice_len;

            // Compute next values
            let ptr = if pre_slice.is_empty() {
                self.current_working_slice.as_ptr()
            } else {
                self.pre_slice.as_ptr()
            };

            self.pre_slice = unsafe {
                &*core::slice::from_raw_parts(ptr, next_pre_slice_len)
            };

            (self.current_working_slice, self.post_slice) = (
                &self.post_slice[..clamped_next_post_slice_len],
                &self.post_slice[clamped_next_post_slice_len..],
            );

            Some((pre_slice, current_slice, post_slice))
        }
    }

    /// Get size hint from iterator
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.current_working_slice.is_empty() {
            (0, Some(0))
        } else {
            let working_length = 1
                + (self.post_slice.len() + self.window_size - 1)
                    / self.window_size;
            (working_length, Some(working_length))
        }
    }
}

#[derive(Debug)]
pub struct ChunksWithPrePostSliceMut<'a, T> {
    /// Prefix mutable slice ptr of a given slice
    pre_slice: *mut [T],
    /// Current mutable slice ptr of a given slice
    current_working_slice: *mut [T],
    /// Post slice to work on
    post_slice: *mut [T],
    /// Window size to be worked on
    window_size: usize,
    /// Prefix len to be manually tracked
    pre_len: usize,
    /// Current len to be manually tracked
    ///
    /// can be different from [`window_size`] when `slice.len() < window_size`
    curr_len: usize,
    /// Post len, tracking [`post_slice`] len
    rem_len: usize,
    /// Marker
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ChunksWithPrePostSliceMut<'a, T> {
    #[inline]
    pub fn new(slice: &'a mut [T], window_size: usize) -> Self {
        let clamped_size = core::cmp::min(window_size, slice.len());
        Self {
            pre_slice: &mut [],
            current_working_slice: &mut slice[..clamped_size],
            post_slice: &mut slice[clamped_size..],
            window_size,
            pre_len: 0,
            curr_len: clamped_size,
            rem_len: slice.len() - clamped_size,
            _marker: PhantomData,
        }
    }

    /// Get length of pre-slice
    #[inline(always)]
    fn prefix_len(&self) -> usize {
        self.pre_len
    }

    /// Get length of current slice
    #[inline(always)]
    fn curr_len(&self) -> usize {
        self.curr_len
    }

    /// Get length of post slice
    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.rem_len
    }
}

impl<'a, T: 'a> Iterator for ChunksWithPrePostSliceMut<'a, T> {
    type Item = (&'a mut [T], &'a mut [T], &'a mut [T]);

    /// Get the next slice, with complete pre-slice and post-slice
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_len() == 0 {
            None
        } else {
            // current slice len to be returned
            let prev_returned_slice_len = self.curr_len;
            // Retrieve the slices that user currently gets,
            // then next slice will be computed
            let (pre_slice, current_slice, post_slice) =
                (self.pre_slice, self.current_working_slice, self.post_slice);

            // Length of next current slice
            let clamped_next_slice_len =
                core::cmp::min(self.window_size, self.remaining_len());

            // Length of post slice
            let clamped_next_post_slice_start =
                core::cmp::min(self.window_size, self.remaining_len());

            let (ptr, post_ptr) = (
                self.current_working_slice as *mut T,
                self.post_slice as *mut T,
            );
            let pre_ptr = if self.prefix_len() > 0 {
                self.pre_slice as *mut T
            } else {
                ptr
            };

            self.curr_len = clamped_next_slice_len;
            self.pre_len += prev_returned_slice_len;
            self.rem_len -= clamped_next_slice_len;

            self.pre_slice =
                core::ptr::slice_from_raw_parts_mut(pre_ptr, self.pre_len);

            self.current_working_slice = unsafe {
                core::ptr::slice_from_raw_parts_mut(
                    ptr.add(prev_returned_slice_len),
                    self.curr_len,
                )
            };
            self.post_slice = unsafe {
                core::ptr::slice_from_raw_parts_mut(
                    post_ptr.add(clamped_next_post_slice_start),
                    self.rem_len,
                )
            };

            unsafe {
                Some((&mut *pre_slice, &mut *current_slice, &mut *post_slice))
            }
        }
    }

    // Get size hint from iterator
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.curr_len() == 0 {
            (0, Some(0))
        } else {
            let working_length =
                1 + (self.rem_len + self.window_size - 1) / self.window_size;
            (working_length, Some(working_length))
        }
    }
}

impl<'a, T: 'a> DoubleEndedIterator for ChunksWithPrePostSliceMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
