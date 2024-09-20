use std::fmt::Debug;
use std::cmp::{min, max};
use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_min_max_in_slice;

pub struct StableBidirectionalSelectionSort {}

impl Default for StableBidirectionalSelectionSort {
    fn default() -> StableBidirectionalSelectionSort {
        return StableBidirectionalSelectionSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for StableBidirectionalSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let mut unsorted_end = vec.len() - 1;

        while unsorted_start < unsorted_end {
            // Find minimum and maximum value
            let ((min_idx, min_val), (max_idx, max_val)) = find_min_max_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();
            
            // Find the location of the left and right hole 
            let left_hole_idx = min(min_idx, max_idx);
            let right_hole_idx = max(min_idx, max_idx);
            
            // Rotate respectively
            vec[unsorted_start..=(unsorted_start + left_hole_idx)].rotate_right(1);
            vec[(unsorted_start + right_hole_idx)..=unsorted_end].rotate_left(1);
            
            // Insert minimum and maximum value into their respective spots
            vec[unsorted_start] = min_val;
            vec[unsorted_end] = max_val;
            
            unsorted_start += 1;
            unsorted_end -= 1;
        }
    }

    fn name(&self) -> String {
        return String::from("Stable Bidirectional Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SBS");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}