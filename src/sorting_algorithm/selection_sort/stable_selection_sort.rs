use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_min_in_slice;

pub struct StableSelectionSort {}

impl Default for StableSelectionSort {
    fn default() -> StableSelectionSort {
        return StableSelectionSort {};
    }
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for StableSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let unsorted_end = vec.len() - 1;

        while unsorted_start < unsorted_end {
            let (min_idx, min_val) = find_min_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();
            
            // Rotate the appropriate elements to the right
            vec[unsorted_start..=(unsorted_start + min_idx)].rotate_right(1);
            vec[unsorted_start] = min_val;
            
            unsorted_start += 1;
        }
        return;
    }

    fn name(&self) -> String {
        return String::from("Stable Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SS");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}