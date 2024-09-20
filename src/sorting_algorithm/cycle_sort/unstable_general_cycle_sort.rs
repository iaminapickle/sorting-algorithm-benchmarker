use std::fmt::Debug;
use std::mem::swap;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::sorting_algorithm::cycle_sort::{generate_count_map, generate_cumulative_count_map};

pub struct UnstableGeneralCycleSort {}

impl Default for UnstableGeneralCycleSort {
    fn default() -> UnstableGeneralCycleSort {
        return UnstableGeneralCycleSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableGeneralCycleSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let count_map = generate_count_map(vec);

        // Generate the cumulative count vector
        // This is a vector such that vec[n] = count(< n)
        let mut cumulative_count_map = generate_cumulative_count_map(&count_map);

        for idx in 0..vec.len() {
            let cycle_start = vec[idx];
            // The correct index of the current element
            let mut correct_idx = cumulative_count_map.get_mut(&vec[idx]).unwrap();
            
            // If the current element is in the wrong spot, rotate an entire cycle once
            if idx >= *correct_idx {
                let mut cur_elem = cycle_start;
                loop {
                    // Increment the correct index for a potential next element
                    correct_idx = cumulative_count_map.get_mut(&cur_elem).unwrap();
                    // Swap with current element with the one at it's correct index
                    swap(&mut cur_elem, &mut vec[*correct_idx]);
                    
                    *correct_idx += 1;
                    // Exit condition: we've rotated around the entire cycle once
                    if cur_elem == cycle_start {
                        break;
                    }
                }
            }
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Unstable General Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("UGC");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}