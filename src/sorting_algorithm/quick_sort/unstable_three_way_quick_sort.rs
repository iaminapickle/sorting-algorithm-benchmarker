use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct UnstableThreeWayQuickSort {}

impl Default for UnstableThreeWayQuickSort {
    fn default() -> UnstableThreeWayQuickSort {
        return UnstableThreeWayQuickSort {};
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
    vec.swap(0, pivot_idx);
 
    let mut l = 0;
    let mut r = vec.len() - 1;

    let mut i = 0;
    while i <= r {
        // If element is less than pivot, swap with l
        if vec[i] < pivot {
            vec.swap(l, i);
            l += 1;
        } 
        // If element is larger than pivot, swap with r
        else if vec[i] > pivot {
            vec.swap(i, r);
            r -= 1;

            // Decrement i to process swapped element
            i -= 1;
        }
        i += 1;
    }
    
    return (l, r);
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for UnstableThreeWayQuickSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() <= 1 { return; }

        let (l, r) = partition(vec);
        self.sort(&mut vec[0..l]);
        self.sort(&mut vec[r + 1..]);
    }

    fn name(&self) -> String {
        return String::from("Unstable Three Way Quick Sort");
    }

    fn nickname(&self) -> String {
        return String::from("UTWQ");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}