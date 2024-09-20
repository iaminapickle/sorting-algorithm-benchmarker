use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct BinaryInsertionSort {}

impl Default for BinaryInsertionSort {
    fn default() -> BinaryInsertionSort {
        return BinaryInsertionSort {};
    }
}

// Returns the successor to the elem, if it exists, otherwise None
fn successor_binary_search<T: Ord + Copy + Debug>(arr: &[T], elem: T) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }

    let arr_len: isize = arr.len() as isize;

    let mut left: isize = 0;
    let mut right: isize = arr_len - 1;
    
    while left <= right {
        let mid: isize = (left + right) / 2;

        if arr[mid as usize] <= elem {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    return if left == arr_len { None } else { Some(left as usize) };
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for BinaryInsertionSort {
    fn sort(&self, vec: &mut [T]) {
        for i in 0..vec.len() {
            let cur = vec[i];
            
            // Search for the successor 
            let idx = successor_binary_search(&vec[..i], cur);
            
            // Shift all elements in the range [idx..i] right by one if necessary
            if idx != None {
                vec[idx.unwrap()..i + 1].rotate_right(1);
                vec[idx.unwrap()] = cur;
            }
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Binary Insertion Sort");
    }

    fn nickname(&self) -> String {
        return String::from("BI");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}