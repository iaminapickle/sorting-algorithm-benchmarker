use std::fmt::Debug;

use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_all_min_in_slice;

pub struct UnstableGroupedSelectionSort {}

impl Default for UnstableGroupedSelectionSort {
    fn default() -> UnstableGroupedSelectionSort {
        return UnstableGroupedSelectionSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableGroupedSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let unsorted_end = vec.len() - 1;
        
        while unsorted_start < unsorted_end {
            let (all_min_vec, _) = find_all_min_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();
            
            let prev = unsorted_start;
            for (min_idx, _) in all_min_vec {
                vec.swap(unsorted_start, prev + min_idx);
                unsorted_start += 1;
            }
        }
    }

    fn name(&self) -> String {
        return String::from("Unstable Grouped Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("UGS");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}