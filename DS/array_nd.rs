use std::fmt;

/// Demo for creating a n-dimensional homogenous array.
/// This is internal for the `NDArray` struct to store
/// info.
/// 
#[derive(Debug, Clone)]
enum NDArrayInternal<T> {
    Array(Vec<NDArrayInternal<T>>),
    Element(T)
}

impl<T> fmt::Display for NDArrayInternal<T> where T:fmt::Display {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.print())
    }
}

impl<T> NDArrayInternal<T> {
    pub fn new() -> NDArrayInternal<T> {
        NDArrayInternal::Array(Vec::new())
    }

    pub fn new_dimensional_array(dimension: &Vec<u16>, init: T) -> NDArrayInternal<T> where T:Copy {
        NDArrayInternal::Array(
            if dimension.len() == 1 {
                let mut arr: Vec<NDArrayInternal<T>> = Vec::new();
            
                for _x in 0..dimension[0] {
                    arr.push(NDArrayInternal::Element(init));
                }
                arr
            } else {
                let mut arr: Vec<NDArrayInternal<T>> = Vec::new();
                let new_dimen = dimension[1..].to_vec();
                for _x in 0..dimension[0] {
                    arr.push(NDArrayInternal::new_dimensional_array(&new_dimen.to_vec(), init));
                }

                arr
            }
        )
    }

    pub fn print(&self) -> String where T:fmt::Display {
        match self {
            NDArrayInternal::Array(ref value) => {
                let mut dstr = "[".to_string();
                for x in 0..value.len() {
                    dstr.push_str(&value[x].print());
                    if x != value.len() - 1 {
                        dstr.push_str(", ");
                    }
                }
                dstr.push(']');
                dstr
            }
            NDArrayInternal::Element(ref value) => {
                value.to_string()
            }
        }
    }
}

#[derive(Debug)]
pub struct NDArray<T> {
    array: NDArrayInternal<T>,
    dimension: u8
}

impl<T> NDArray<T> {
    pub fn new() -> NDArray<T> {
        NDArray {
            array: NDArrayInternal::new(),
            dimension: 0u8
        }
    }

    pub fn create_dimensional_array(dimension: &Vec<u16>, init: T) -> NDArray<T> where T:Copy {
        NDArray {
            array: NDArrayInternal::new_dimensional_array(dimension, init),
            dimension: dimension.len() as u8
        }
    }
}

impl<T> fmt::Display for NDArray<T> where T:fmt::Display {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.array.print())
    }
}

fn main() {
    let tdarray = NDArray::<u8>::create_dimensional_array(&vec![2, 3, 2, 4], 1<<7);
    println!("{}", tdarray);
}
