use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_min_in_slice;

pub struct UnstableSelectionSort {}

impl Default for UnstableSelectionSort {
    fn default() -> UnstableSelectionSort {
        return UnstableSelectionSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let unsorted_end = vec.len() - 1;
        
        while unsorted_start < unsorted_end {
            let (min_idx, _) = find_min_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();
            vec.swap(unsorted_start, min_idx + unsorted_start);
            unsorted_start += 1;
        }
    }

    fn name(&self) -> String {
        return String::from("Unstable Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("US");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}