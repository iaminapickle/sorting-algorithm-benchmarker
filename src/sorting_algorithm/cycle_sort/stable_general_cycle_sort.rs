use std::fmt::Debug;
use std::mem::swap;
use std::hash::Hash;
use crate::sort_elem::{to_sort_elem_vec, SortElemVec};
use crate::sorting_algorithm::SortingAlgorithm;
use crate::sorting_algorithm::cycle_sort::{generate_count_map, generate_cumulative_count_map};

pub struct StableGeneralCycleSort {}

impl Default for StableGeneralCycleSort {
    fn default() -> StableGeneralCycleSort {
        return StableGeneralCycleSort {};
    }
}

impl<T: Ord + Copy + Debug + Hash> SortingAlgorithm<T> for StableGeneralCycleSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }
        
        let mut sort_elem_vec: SortElemVec<T> = to_sort_elem_vec(vec.to_vec());

        // Generate the count BTreeMap
        // This is a BTreeMap such that map[key] = count(== key)
        let count_map = generate_count_map(&sort_elem_vec);

        // Generate the cumulative count BTreeMap
        // This is a BTreeMap such that map[key] = count(< key)
        let mut cumulative_count_map = generate_cumulative_count_map(&count_map);
        for idx in 0..sort_elem_vec.len() {
            let mut cur_sort_elem = sort_elem_vec[idx];

            loop {
                // Increment the correct index for a potential next element
                let correct_idx = *cumulative_count_map.get_mut(&cur_sort_elem).unwrap() + cur_sort_elem.occ;
                
                // Swap with current element with the one at it's correct index
                swap(&mut cur_sort_elem, &mut sort_elem_vec[correct_idx]);

                // Exit condition: we've rotated around the entire cycle once
                if correct_idx == idx {
                    break;
                }
            }
        }

        for idx in 0..vec.len() {
            vec[idx] = sort_elem_vec[idx].val;
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Stable General Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SGC");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}