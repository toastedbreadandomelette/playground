pub fn bubble_sort<T: std::cmp::PartialOrd + Copy>(array: &mut Vec<T>) {
    for out in (0..array.len()).rev() {
        for i in 0..out {
            if array[i] > array[i + 1] {
                (array[i], array[i + 1]) = (array[i + 1], array[i]);
            }
        }
    }
}

pub fn bubble_sort_cmp<T: std::cmp::PartialOrd + Copy>(
    array: &mut Vec<T>,
    cmp: &dyn Fn(T, T) -> bool,
) {
    for out in (0..array.len()).rev() {
        for i in 0..out {
            if !cmp(array[i], array[i + 1]) {
                (array[i], array[i + 1]) = (array[i + 1], array[i]);
            }
        }
    }
}

pub fn selection_sort<T: std::cmp::PartialOrd + Copy>(array: &mut Vec<T>) {
    for out in 0..array.len() {
        let min = (out..array.len())
            .into_iter()
            .enumerate()
            .reduce(|p, c| if array[p.1] > array[c.1] { c } else { p })
            .unwrap();
        (array[out], array[min.1]) = (array[min.1], array[out]);
    }
}

pub fn selection_sort_cmp<T: std::cmp::PartialOrd + Copy>(
    array: &mut Vec<T>,
    cmp: &dyn Fn(T, T) -> bool,
) {
    for out in 0..array.len() {
        let min = (out..array.len())
            .into_iter()
            .enumerate()
            .reduce(|p, c| if !cmp(array[p.1], array[c.1]) { c } else { p })
            .unwrap();
        (array[out], array[min.1]) = (array[min.1], array[out]);
    }
}

pub fn insertion_sort<T: std::cmp::PartialOrd + Copy>(array: &mut Vec<T>) {
    for out in 1..array.len() {
        let mut index = out;
        while index > 0 && array[index] < array[index - 1] {
            (array[index], array[index - 1]) = (array[index - 1], array[index]);
            index -= 1;
        }
    }
}

pub fn insertion_sort_cmp<T: std::cmp::PartialOrd + Copy>(
    array: &mut Vec<T>,
    cmp: &dyn Fn(T, T) -> bool,
) {
    for out in 1..array.len() {
        let mut index = out;
        while index > 0 && !cmp(array[index - 1], array[index]) {
            (array[index], array[index - 1]) = (array[index - 1], array[index]);
            index -= 1;
        }
    }
}

// A non-recursive quick sort implementation
pub fn quick_sort<T: std::cmp::PartialOrd + Copy>(array: &mut Vec<T>) {
    let mut sort_stack: Vec<(usize, usize)> = Vec::new();
    sort_stack.push((0, array.len() - 1));
    while sort_stack.len() > 0 {
        let (mut i, mut j) = sort_stack.pop().unwrap();
        if j <= i {
            continue;
        }
        if j - i > 4 {
            let pivot = array[i];
            let pivot_index = i;
            i = i + 1;
            let (start, end) = (i, j);
            for x in start..=end {
                if pivot > array[x] {
                    (array[i], array[x]) = (array[x], array[i]);
                    i += 1;
                }
            }
            if i > 0 {
                (array[pivot_index], array[i - 1]) = (array[i - 1], array[pivot_index]);
            }
            if i > 1 {
                sort_stack.push((pivot_index, i - 2));
            }
            sort_stack.push((i, end));
        } else {
            for out in i + 1..=j {
                let mut index = out;
                while index > i && array[index] < array[index - 1] {
                    (array[index], array[index - 1]) = (array[index - 1], array[index]);
                    index -= 1;
                }
            }
        }
    }
}

// A non-recursive quick sort implementation
pub fn quick_sort_cmp<T: std::cmp::PartialOrd + Copy>(
    array: &mut Vec<T>,
    cmp: &dyn Fn(T, T) -> bool,
) {
    let mut sort_stack: Vec<(usize, usize)> = Vec::new();
    sort_stack.push((0, array.len() - 1));
    while sort_stack.len() > 0 {
        let (mut i, mut j) = sort_stack.pop().unwrap();
        if j <= i {
            continue;
        }
        if j - i > 4 {
            let pivot = array[i];
            let pivot_index = i;
            i = i + 1;
            let (start, end) = (i, j);
            for x in start..=end {
                if !cmp(pivot, array[x]) {
                    (array[i], array[x]) = (array[x], array[i]);
                    i += 1;
                }
            }
            if i > 0 {
                (array[pivot_index], array[i - 1]) = (array[i - 1], array[pivot_index]);
            }
            if i > 1 {
                sort_stack.push((pivot_index, i - 2));
            }
            sort_stack.push((i, end));
        } else {
            for out in i + 1..=j {
                let mut index = out;
                while index > i && !cmp(array[index - 1], array[index]) {
                    (array[index], array[index - 1]) = (array[index - 1], array[index]);
                    index -= 1;
                }
            }
        }
    }
}

#[test]
pub fn test_bubble() {
    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    bubble_sort(&mut s);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 123, 132, 556]);
}

#[test]
pub fn test_bubble_cmp() {
    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    bubble_sort_cmp(&mut s, &|a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 123, 132, 556]);

    let count_bits = |mut c| {
        let mut count = 0;
        while c > 0 {
            count += c & 1;
            c >>= 1;
        }
        return count;
    };

    // Sort values according to most bits in the number
    // If bits are same then sort according to their magnitude
    s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    bubble_sort_cmp(&mut s, &|a, b| {
        (count_bits(a) < count_bits(b) || (count_bits(a) == count_bits(b) && a < b))
    });
    assert_eq!(s, [1, 2, 4, 3, 5, 6, 132, 556, 123]);
}

#[test]
pub fn test_select() {
    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    selection_sort(&mut s);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 123, 132, 556]);
}

#[test]
pub fn test_select_cmp() {
    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    selection_sort_cmp(&mut s, &|a, b| a < b);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 123, 132, 556]);

    let count_bits = |mut c| {
        let mut count = 0;
        while c > 0 {
            count += c & 1;
            c >>= 1;
        }
        return count;
    };

    // Sort values according to most bits in the number
    // If bits are same then sort according to their magnitude
    s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    selection_sort_cmp(&mut s, &|a, b| {
        count_bits(a) < count_bits(b) || (count_bits(a) == count_bits(b) && a < b)
    });
    assert_eq!(s, [1, 2, 4, 3, 5, 6, 132, 556, 123]);
}

#[test]
pub fn test_insertion() {
    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    insertion_sort(&mut s);
    assert_eq!(s, [1, 2, 3, 4, 5, 6, 123, 132, 556]);
}

#[test]
pub fn test_quick() {
    let count_bits = |mut c| {
        let mut count = 0;
        while c > 0 {
            count += c & 1;
            c >>= 1;
        }
        return count;
    };

    let mut s = vec![4, 3, 2, 1, 6, 5, 123, 132, 556];
    quick_sort_cmp(&mut s, &|a, b| {
        count_bits(a) < count_bits(b) || (count_bits(a) == count_bits(b) && a < b)
    });
    assert_eq!(s, [1, 2, 4, 3, 5, 6, 132, 556, 123]);
}

// A non-recursive merge sort
// At base insertion sort is used (for e.g., subarray of window size 16),
// and then working way upto larger sizes until we're in final stage of merging two
// array partitions
pub fn merge_sort<
    T: std::cmp::PartialOrd + Copy + std::default::Default + std::convert::From<T>,
>(
    arr: &mut Vec<T>,
    cmp: &dyn Fn(T, T) -> bool,
) {
    // Placeholder, this takes care of copying the arrangment of array.
    let mut placeholder: Vec<T> = vec![std::default::Default::default(); arr.len()];

    // sort 16 length of block by insertion method, then merge
    // them, for that placeholder is the array used to keep them
    let mut block_size = 8;
    for x in (0..arr.len()).step_by(block_size) {
        let end = if x + block_size > arr.len() {
            arr.len()
        } else {
            x + block_size
        };
        for y in (x + 1)..end {
            let mut index = y;
            while index > x && !cmp(arr[index - 1], arr[index]) {
                (arr[index], arr[index - 1]) = (arr[index - 1], arr[index]);
                index -= 1;
            }
        }
    }

    while block_size < arr.len() {
        for x in (0..arr.len()).step_by(2 * block_size) {
            if x + block_size > arr.len() {
                break;
            } else {
                let (start, mid) = (x, x + block_size);
                let end = if x + 2 * block_size > arr.len() {
                    arr.len()
                } else {
                    x + 2 * block_size
                };
                let (mut fptr, mut sptr, mut pptr) = (start, mid, start);
                while fptr < mid && sptr < end {
                    if cmp(arr[fptr], arr[sptr]) {
                        placeholder[pptr] = arr[fptr];
                        pptr += 1;
                        fptr += 1;
                    } else {
                        placeholder[pptr] = arr[sptr];
                        pptr += 1;
                        sptr += 1;
                    }
                }
                while fptr < mid {
                    placeholder[pptr] = arr[fptr];
                    pptr += 1;
                    fptr += 1;
                }
                // This step might not be required, since we know they are in ascending order
                // stored in array, so it is not necessary to copy.
                // while sptr < end {
                //     placeholder[pptr] = arr[sptr];
                //     pptr += 1;
                //     sptr += 1;
                // }

                // More optimization, alternating placeholder and arr instead of
                // doing this.
                for ret_ptr in start..pptr {
                    arr[ret_ptr] = placeholder[ret_ptr];
                }
            }
        }

        block_size <<= 1;
    }
}

#[test]
pub fn test_merge_sort() {
    let mut s = (0..=1000000)
        .into_iter()
        .rev()
        .map(|x| x as u32)
        .collect::<Vec<u32>>();
    merge_sort(&mut s, &|curr, next| curr < next);
    assert_eq!(
        s,
        (0..=1000000)
            .into_iter()
            .map(|x| x as u32)
            .collect::<Vec<u32>>()
    );
}

fn main() {
    let sz = 10000000;
    let mut s = (0..=sz)
        .into_iter()
        .rev()
        .map(|x| x as u32)
        .collect::<Vec<u32>>();
    merge_sort(&mut s, &|curr, next| curr < next);
    assert_eq!(
        (0..=sz).into_iter().map(|x| x as u32).collect::<Vec<u32>>(),
        s
    );
}
