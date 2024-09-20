use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct MergeSort {}

impl Default for MergeSort {
    fn default() -> MergeSort {
        return MergeSort {};
    }
}

pub fn merge<T:Ord + Copy + Debug>(a: &[T], b: &[T]) -> Vec<T> {
    let mut a_idx = 0;
    let mut b_idx = 0;

    let mut c = Vec::with_capacity(a.len() + b.len());

    while a_idx != a.len() && b_idx != b.len() {
        if a[a_idx] <= b[b_idx] {
            c.push(a[a_idx]);
            a_idx += 1;        
        } else {
            c.push(b[b_idx]);
            b_idx += 1;
        }
    } 
    
    while a_idx != a.len() {
        c.push(a[a_idx]);
        a_idx += 1;
    }
    
    while b_idx != b.len() {
        c.push(b[b_idx]);
        b_idx += 1;
    }
    return c
}


fn merge_sort<T:Ord + Copy + Debug>(vec: &[T]) -> Vec<T> {
    if vec.len() == 1 {
        return vec.to_vec();
    }

    let mid = vec.len() / 2;

    let left = merge_sort(&vec[..mid]);
    let right = merge_sort(&vec[mid..]);

    return merge(&left, &right);
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for MergeSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }
        vec.copy_from_slice(&merge_sort(&vec));
    }

    fn name(&self) -> String {
        return String::from("Merge Sort");
    }

    fn nickname(&self) -> String {
        return String::from("M");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}