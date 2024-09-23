use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct QuickSort {}

impl Default for QuickSort {
    fn default() -> QuickSort {
        return QuickSort {};
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

// Using Hoare's Partitioning algorithm
fn partition<T:Ord + Copy + Debug>(vec: &mut [T]) -> usize {
    // Choose median of three as pivot
    let pivot_idx = choose_pivot(vec);
    let pivot =  vec[pivot_idx];
    vec.swap(0, pivot_idx);
 
    let mut l: isize = -1;
    let mut r = vec.len();

    loop {
        // Increment left index until an element is greater than or equal to the pivot
        while {
            l += 1;
            
            vec[l as usize] < pivot
        } {}
        
        // Increment right index until an element is less than or equal to the pivot
        while {
            r -= 1;

            vec[r] > pivot
        } {}

        if l as usize >= r {
            return r;
        }

        vec.swap(l as usize, r); 
    }
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for QuickSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() <= 1 { return; }

        let p = partition(vec);
        self.sort(&mut vec[0..p + 1]);
        self.sort(&mut vec[p + 1..]);
    }

    fn name(&self) -> String {
        return String::from("Quick Sort");
    }

    fn nickname(&self) -> String {
        return String::from("Q");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}