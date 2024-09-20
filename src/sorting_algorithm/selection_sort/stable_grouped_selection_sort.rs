use std::fmt::Debug;

use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_all_min_in_slice;

pub struct StableGroupedSelectionSort {}

impl Default for StableGroupedSelectionSort {
    fn default() -> StableGroupedSelectionSort {
        return StableGroupedSelectionSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for StableGroupedSelectionSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut unsorted_start = 0;
        let unsorted_end = vec.len() - 1;

        while unsorted_start < unsorted_end {
            // Find all minimum values
            let (all_min_vec, _) = find_all_min_in_slice(&vec[unsorted_start..=unsorted_end]).unwrap();

            let new_unsorted_start = unsorted_start + all_min_vec.len();
            let largest_min_idx: usize = unsorted_start + all_min_vec.last().unwrap().0; 
            let mut all_min_rev = all_min_vec.iter().rev().peekable();
            
            // Shift all elements across to fill all the gaps
            let mut offset = 0;
            for n in (new_unsorted_start..=largest_min_idx).rev() {
                while all_min_rev.peek() != None && all_min_rev.peek().unwrap().0 == n - offset - unsorted_start {
                    offset += 1;
                    all_min_rev.next();
                }
                vec[n] = vec[n - offset];
            }
            
            // Copy the min elements to the end of the sorted section
            for (_, min) in all_min_vec {
                vec[unsorted_start] = min;
                unsorted_start += 1;
            }
        }
    }

    fn name(&self) -> String {
        return String::from("Stable Grouped Selection Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SGS");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}
