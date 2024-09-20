use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_min_max_in_slice;

pub struct UnstableBidirectionalSelectionSort {}

impl Default for UnstableBidirectionalSelectionSort {
    fn default() -> UnstableBidirectionalSelectionSort {
        return UnstableBidirectionalSelectionSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableBidirectionalSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let mut unsorted_end = vec.len() - 1;

        while unsorted_start < unsorted_end {
            let ((min_idx, _), (mut max_idx, _)) = find_min_max_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();

            // Maximum value will not be here after the min swap
            if max_idx == 0 {
                max_idx = min_idx;
            }

            vec.swap(unsorted_start, unsorted_start + min_idx);
            unsorted_start += 1;

            vec.swap(unsorted_end, unsorted_start + max_idx - 1);
            unsorted_end -= 1;
        }
    }

    fn name(&self) -> String {
        return String::from("Unstable Bidirectional Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("UBS");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}