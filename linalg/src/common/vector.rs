use core::alloc::Layout;
use core::ops::{
    Deref, DerefMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};
use core::ops::{Index, IndexMut};
use core::slice::{
    Chunks, ChunksExact, ChunksExactMut, ChunksMut, Iter, IterMut, Windows,
};

#[cfg(feature = "array_chunks")]
use core::slice::VectorChunks;
#[cfg(feature = "array_chunks")]
use core::slice::VectorChunksMut;
#[cfg(feature = "array_windows")]
use core::slice::VectorWindows;

extern crate alloc;

/// A custom vector for `![no_std]`
///
/// This struct will have constant heap allocated value for type `T`
/// and would only implement traits for following types:
/// - `Index` (for type `usize`, `Range`, `RangeInclusive` (and maybe `isize`))
/// - `IndexMut` (for type `usize`, `Range`, `RangeInclusive` (and maybe `isize`))
/// - `Iterator`
pub struct Vector<T> {
    /// Pointer to the array
    ptr: *mut T,
    /// Layout defined by the declared type
    /// This contains the total size in bytes with the alignment
    layout: Layout,
    /// Capacity of an array to hold
    capacity: usize,
    /// Actual length of an array.
    len: usize,
}

impl<T> Vector<T> {
    #[inline(always)]
    fn align_nearest_to(val: usize) -> usize {
        (((val + 31) >> 5) << 5) * core::mem::size_of::<T>()
    }

    fn calc_capacity(val: usize) -> usize {
        ((val + 31) >> 5) << 5
    }
    /// Create a new array, this is a custom made array
    /// with an intention to use no_std allocation
    #[inline]
    pub fn new(len: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                Self::align_nearest_to(len),
                32,
            );
            Self {
                ptr: alloc::alloc::alloc(layout).cast::<T>(),
                layout,
                len,
                capacity: Self::calc_capacity(len),
            }
        }
    }

    /// Mutate and add an element into array to fit the size internally.
    ///
    /// The alignment is strictly kept as `32`.
    fn mutate_add(&mut self, element: T) {
        if self.len == 0 {
            *self = Self::zeroed(1);
        } else {
            if self.len == self.capacity {
                // Allocate 2 times the current size of memory.
                // A new layout is created here
                let new_layout = unsafe {
                    let new_byte_size = Self::align_nearest_to(self.len * 2);
                    Layout::from_size_align_unchecked(new_byte_size, 32)
                };

                self.ptr = unsafe {
                    // Documentation states that this will be deprecated.
                    let new_ptr =
                        alloc::alloc::alloc_zeroed(new_layout).cast::<T>();
                    core::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                    alloc::alloc::dealloc(self.ptr.cast::<u8>(), self.layout);
                    new_ptr
                };
                self.layout = new_layout;
                self.capacity = Self::calc_capacity(self.len * 2);
            }
            // Finally write to the pointer
            unsafe { self.ptr.add(self.len).write(element) };
            self.len += 1;
        }
    }

    /// Mutates the capacity by extra capacity. Also considers the excess `capacity`
    /// the `self` has and increases appropriately
    #[inline(always)]
    pub fn mutate_capacity_by(&mut self, extra_capacity: usize) {
        // If the extra capacity requested is greater then no need
        // of extending the array.
        if self.capacity < self.len + extra_capacity {
            // Remove the excess capacity
            self.mutate_capacity_by_ignore_current(
                extra_capacity - (self.capacity - self.len),
            );
        }
    }

    /// Mutates the capacity of array
    ///
    /// Note that this does not take into consideration the current
    /// capacity and adds the value. To prevent unnecessary `extra_capacity`,
    /// refer `mutate_capacity_by`
    ///
    /// The alignment is strictly kept to `32`.
    pub fn mutate_capacity_by_ignore_current(&mut self, extra_capacity: usize) {
        if self.len == 0 {
            *self = Self::zeroed(extra_capacity);
        } else {
            // Allocate precise extra capacity, with extra space for alignment
            let new_layout = unsafe {
                let new_byte_size =
                    Self::align_nearest_to(self.capacity + extra_capacity);
                Layout::from_size_align_unchecked(new_byte_size, 32)
            };

            self.ptr = unsafe {
                // Documentation states that this will be deprecated.
                let new_ptr =
                    alloc::alloc::alloc_zeroed(new_layout).cast::<T>();
                core::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                alloc::alloc::dealloc(self.ptr.cast::<u8>(), self.layout);
                new_ptr
            };
            self.layout = new_layout;
            self.capacity = Self::calc_capacity(self.capacity + extra_capacity);
        }
    }

    /// Extend from an iterator with size hint.
    #[inline]
    pub fn extend_from_iter(&mut self, other_iter: &mut Iter<'_, T>)
    where
        T: Copy,
    {
        let (lower_bound, _) = other_iter.size_hint();
        // Mutate capacity by certain at least `lower_bound`
        self.mutate_capacity_by(lower_bound);
        // Reallocation can be done if extra iter needed
        other_iter.for_each(|f| self.mutate_add(*f));
    }

    /// Length of the array
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Capacity of array
    #[inline(always)]
    pub fn cap(&self) -> usize {
        self.capacity
    }

    /// Returns the borrowed values from iterator
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len).iter() }
    }

    /// Returns the mutable borrowed iterator
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len).iter_mut()
        }
    }

    /// Returns the chunks of `n` size
    ///
    /// Remainder will yield array of `len < chunk_size`
    #[inline(always)]
    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, T> {
        unsafe {
            core::slice::from_raw_parts(self.ptr, self.len).chunks(chunk_size)
        }
    }

    /// Returns the chunks of exactly `n` size
    ///
    /// Remainder can be accessed from `remainder`
    #[inline(always)]
    pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T> {
        unsafe {
            core::slice::from_raw_parts(self.ptr, self.len)
                .chunks_exact(chunk_size)
        }
    }

    /// Returns the mutable slice of `n` size
    ///
    /// Remainder will yield array of `len < chunk_size`
    #[inline(always)]
    pub fn chunks_mut(&self, chunk_size: usize) -> ChunksMut<'_, T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .chunks_mut(chunk_size)
        }
    }

    /// Returns the mutable chunks of exactly `n` size
    ///
    /// Remainder can be accessed from `remainder`
    #[inline(always)]
    pub fn chunks_exact_mut(&self, chunk_size: usize) -> ChunksExactMut<'_, T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .chunks_exact_mut(chunk_size)
        }
    }

    /// Returns the window iterator of exactly `n` size, iterating through
    /// array with single step
    ///
    /// If window size is larger, then it won't yield a slice.
    #[inline(always)]
    pub fn windows(&self, window_size: usize) -> Windows<'_, T> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .windows(window_size)
        }
    }

    /// Returns the window iterator of exactly `n` size, iterating through
    /// array with single step
    ///
    /// If window size is larger, then it won't yield a slice.
    #[inline(always)]
    #[cfg(feature = "array_chunks")]
    pub fn array_chunks<const SIZE: usize>(&self) -> VectorChunks<'_, T, SIZE> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .array_chunks::<SIZE>()
        }
    }

    /// Returns the window iterator of exactly `n` size, iterating through
    /// array with single step
    ///
    /// If window size is larger, then it won't yield a slice.
    #[inline(always)]
    #[cfg(feature = "array_chunks")]
    pub fn array_chunks_mut<const SIZE: usize>(
        &self,
    ) -> VectorChunksMut<'_, T, SIZE> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .array_chunks_mut::<SIZE>()
        }
    }

    /// Returns the static array window of exactly size `SIZE`
    #[inline(always)]
    #[cfg(feature = "array_windows")]
    pub fn array_windows<const SIZE: usize>(
        &self,
    ) -> VectorWindows<'_, T, SIZE> {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr, self.len)
                .array_windows::<SIZE>()
        }
    }

    /// Create a new array filled with value, this is a custom made array
    /// with an intention to use no_std allocation
    #[inline]
    pub fn zeroed(len: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                Self::align_nearest_to(len),
                32,
            );
            Self {
                ptr: alloc::alloc::alloc_zeroed(layout).cast::<T>(),
                layout,
                len,
                capacity: Self::calc_capacity(len),
            }
        }
    }

    /// Create a new array filled with value, this is a custom made array
    /// with an intention to use `![no_std]` allocation
    pub fn from_range(range: Range<T>) -> Self
    where
        T: core::iter::Step + core::ops::Sub<Output = T> + Into<usize> + Copy,
        usize: core::convert::From<T>,
    {
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                Self::align_nearest_to(usize::from(range.end - range.start)),
                32,
            );
            let ptr = alloc::alloc::alloc(layout).cast::<T>();
            let (start, end) = (range.start, range.end);
            for value in range {
                let ptr_offset = ptr.add(usize::from(value - start));
                *ptr_offset = value;
            }
            Self {
                ptr,
                layout,
                len: (end - start).into(),
                capacity: Self::calc_capacity((end - start).into()),
            }
        }
    }
}

impl<T> PartialEq for Vector<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other).all(|(a, b)| a == b)
    }
}

// /// Iterator for Vector
// pub struct VectorIterator<'a, T> {
//     arr: &'a Vector<T>,
//     curr_index: usize,
// }

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Newly created array, with size zero
        let mut new_array: Vector<T> = Vector::zeroed(0);

        // Unfortunately this is extended similar to vector
        // and returned to the user, as there is no hint regarding
        // total elements generated from the iterator
        for item in iter {
            new_array.mutate_add(item);
        }
        new_array
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    // Into Iterator for &'a Vector<T>
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// impl<T> IntoIterator for Vector<T> {
//     type Item = T;
//     type IntoIter = core::
// }

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len);
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self[..]
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self[..]
    }
}

impl<T> Index<Range<usize>> for Vector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, range: Range<usize>) -> &Self::Output {
        debug_assert!(range.start < self.len && range.end <= self.len);
        unsafe {
            core::slice::from_raw_parts(
                self.ptr.add(range.start),
                range.end - range.start,
            )
        }
    }
}

impl<T> Index<RangeFrom<usize>> for Vector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        debug_assert!(range.start < self.len);
        unsafe {
            core::slice::from_raw_parts(
                self.ptr.add(range.start),
                self.len - range.start,
            )
        }
    }
}

impl<T> Index<core::ops::RangeInclusive<usize>> for Vector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        debug_assert!(*range.start() < self.len && *range.end() < self.len);
        unsafe {
            core::slice::from_raw_parts(
                self.ptr.add(*range.start()),
                *range.end() - *range.start(),
            )
        }
    }
}

impl<T> Index<core::ops::RangeTo<usize>> for Vector<T> {
    type Output = [T];
    #[inline]
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        debug_assert!(range.end < self.len);
        unsafe { core::slice::from_raw_parts(self.ptr, range.end) }
    }
}

impl<T> Index<core::ops::RangeToInclusive<usize>> for Vector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        debug_assert!(range.end < self.len);
        unsafe { core::slice::from_raw_parts(self.ptr, range.end) }
    }
}

impl<T> Index<RangeFull> for Vector<T> {
    type Output = [T];

    #[inline]
    fn index(&self, _: RangeFull) -> &Self::Output {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> IndexMut<Range<usize>> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, range: Range<usize>) -> &mut [T] {
        debug_assert!(range.start < self.len && range.end < self.len);
        unsafe {
            &mut *core::slice::from_raw_parts_mut(
                self.ptr.add(range.start),
                range.end - range.start,
            )
        }
    }
}

impl<T> IndexMut<RangeFrom<usize>> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut [T] {
        debug_assert!(range.start < self.len);
        unsafe {
            &mut *core::slice::from_raw_parts_mut(
                self.ptr.add(range.start),
                self.len - range.start,
            )
        }
    }
}

impl<T> IndexMut<core::ops::RangeInclusive<usize>> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut [T] {
        debug_assert!(*range.start() < self.len && *range.end() < self.len);
        unsafe {
            &mut *core::slice::from_raw_parts_mut(
                self.ptr.add(*range.start()),
                *range.end() - *range.start(),
            )
        }
    }
}

impl<T> IndexMut<core::ops::RangeTo<usize>> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut [T] {
        debug_assert!(range.end < self.len);
        unsafe { &mut *core::slice::from_raw_parts_mut(self.ptr, range.end) }
    }
}

impl<T> IndexMut<core::ops::RangeToInclusive<usize>> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut [T] {
        debug_assert!(range.end < self.len);
        unsafe { &mut *core::slice::from_raw_parts_mut(self.ptr, range.end) }
    }
}

impl<T> IndexMut<RangeFull> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, _: RangeFull) -> &mut [T] {
        unsafe { &mut *core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.len);
        unsafe { &mut *self.ptr.add(index) }
    }
}

impl<T> Drop for Vector<T> {
    #[inline]
    fn drop(&mut self) {
        // A simple deallocation
        unsafe { alloc::alloc::dealloc(self.ptr.cast::<u8>(), self.layout) }
    }
}
