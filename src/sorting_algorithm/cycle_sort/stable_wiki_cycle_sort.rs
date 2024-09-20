use std::fmt::Debug;
use std::mem::swap;
use std::hash::Hash;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::sort_elem::{to_sort_elem_vec};

pub struct StableWikiCycleSort {}

impl Default for StableWikiCycleSort {
    fn default() -> StableWikiCycleSort {
        return StableWikiCycleSort {};
    }
}

impl<T: Ord + Copy + Debug + Hash> SortingAlgorithm<T> for StableWikiCycleSort {
    fn sort(&self, vec: &mut [T]) {
        // This is technically useless because the vector is already pre-processed before entering this sort function, 
        // thus, this is included to reflect the time the processing would take in a real environment.
        let mut sort_elem_vec = to_sort_elem_vec(vec.to_vec());

        for idx in 0..sort_elem_vec.len() {
            let mut cur_idx = idx;
            let mut cur_sort_elem = sort_elem_vec[idx];
            
            loop {
                // Calculate such that correct_idx = count( < cur_elem.val ) + cur_elem.occ - 1
                // All elements < idx are in their correct positions and thus, are smaller than any unsorted numbers
                let mut correct_idx = idx + sort_elem_vec[idx + 1..].iter().filter(|&n| n.val < cur_sort_elem.val).count() + cur_sort_elem.occ;

                // Exit condition: cur_elem is already in the correct position
                if cur_idx == correct_idx {
                    break;
                }
                
                // Starting from idx - 1 and iterating leftwards, decrement correct_idx for every cur_elem value
                let mut j = idx;
                while j != 0 && sort_elem_vec[j - 1].val == cur_sort_elem.val {
                    correct_idx -= 1;
                    j -= 1;
                }
                
                swap(&mut cur_sort_elem, &mut sort_elem_vec[correct_idx]);
                cur_idx = correct_idx;
                
                // Exit condition: we've rotated around the entire cycle once
                if cur_idx == idx {
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
        return String::from("Stable Wiki Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SWC");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}