use std::fmt::Debug;
use std::mem::swap;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct UnstableWikiCycleSort {}

impl Default for UnstableWikiCycleSort {
    fn default() -> UnstableWikiCycleSort {
        return UnstableWikiCycleSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableWikiCycleSort {
    fn sort(&self, vec: &mut [T]) {
        for idx in 0..vec.len() {
            let mut cur_idx = idx;
            let mut cur_elem = vec[idx];
            
            loop {
                // Calculate such that correct_idx = count( < cur_elem )
                // All elements < idx are in their correct positions and thus, are not larger than any unsorted numbers
                let mut correct_idx = idx + vec[idx + 1..].iter().filter(|&n| *n < cur_elem).count();

                // Exit condition: cur_elem is already in the correct position
                if cur_idx == correct_idx { 
                    break;
                }

                // Starting from correct_idx and iterating rightwards, increment correct_idx for every cur_elem value
                while vec[correct_idx] == cur_elem {
                    correct_idx += 1;
                }
                
                swap(&mut cur_elem, &mut vec[correct_idx]);
                cur_idx = correct_idx;

                // Exit condition: we've rotated around the entire cycle once
                if cur_idx == idx {
                    break;
                }
                
            }
        }
        
        return;
    }

    fn name(&self) -> String {
        return String::from("Unstable Wiki Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("UWC");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}