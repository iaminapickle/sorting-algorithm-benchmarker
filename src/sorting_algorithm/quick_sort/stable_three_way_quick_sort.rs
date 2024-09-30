use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct StableThreeWayQuickSort {}

impl Default for StableThreeWayQuickSort {
    fn default() -> StableThreeWayQuickSort {
        return StableThreeWayQuickSort {};
    }
}

// Returns the index of the pivot - the median of three
fn choose_pivot<T:Ord + Copy + Debug>(vec: &mut [T]) -> usize {
    let mid_idx = vec.len() / 2;
    let candidates = vec![*vec.first().unwrap(), vec[mid_idx], *vec.last().unwrap()];

    if (candidates[0] > candidates[1]) ^ (candidates[0] > candidates[2]) {
        return 0;
    } else if (candidates[1] < candidates[0]) ^ (candidates[1] < candidates[2]) {
        return mid_idx;
    } else {
        return vec.len() - 1;
    }
}

// Partition the vec into [< pivot][= pivot][> pivot]
// Returns the first and last index of the [= pivot] partition
fn partition<T:Ord + Copy + Debug>(vec: &mut [T]) -> (usize, usize) {
    // Choose median of three as pivot
    let pivot_idx = choose_pivot(vec);
    let pivot =  vec[pivot_idx];
     
    // Also used as [< pivot] storage
    let mut less_than_vec = Vec::with_capacity(vec.len());
    let mut greater_than_vec = Vec::with_capacity(vec.len());
    let mut equal_to_vec = Vec::with_capacity(vec.len());

    for n in &mut *vec {
        if *n < pivot {
            less_than_vec.push(*n);
        } else if *n > pivot {
            greater_than_vec.push(*n);
        } else {
            equal_to_vec.push(*n);
        }
    }
    
    let mut idx = 0;
    for n in &less_than_vec {
        vec[idx] = *n;
        idx += 1;
    }

    for n in &equal_to_vec {
        vec[idx] = *n;
        idx += 1;
    }
    
    for n in &greater_than_vec {
        vec[idx] = *n;
        idx += 1;
    }

    let first_idx = less_than_vec.len();
    let last_idx = first_idx + equal_to_vec.len() - 1;
    return (first_idx, last_idx);
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for StableThreeWayQuickSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() <= 1 { return; }

        let (l, r) = partition(vec);
        self.sort(&mut vec[0..l]);
        self.sort(&mut vec[r + 1..]);
    }

    fn name(&self) -> String {
        return String::from("Stable Three Way Quick Sort");
    }

    fn nickname(&self) -> String {
        return String::from("STWQ");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}