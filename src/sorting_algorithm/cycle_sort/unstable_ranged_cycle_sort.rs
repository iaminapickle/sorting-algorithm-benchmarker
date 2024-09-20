use std::fmt::Debug;
use std::mem::swap;
use num::Signed;
use crate::helper::find_min_max_in_slice;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::sorting_algorithm::cycle_sort::{generate_count_vec, generate_cumulative_count_vec};

pub struct UnstableRangedCycleSort {}

impl Default for UnstableRangedCycleSort {
    fn default() -> UnstableRangedCycleSort {
        return UnstableRangedCycleSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableRangedCycleSort where 
    T: TryInto<isize>, <T as TryInto<isize>>::Error: Debug
{
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let ((_, min_elem), (_, max_elem)) = find_min_max_in_slice(vec).unwrap();
        let count_vec = generate_count_vec(vec, min_elem, max_elem);

        let isize_min: isize = min_elem.try_into().unwrap();
        
        // Generate the cumulative count vector
        // This is a vector such that vec[n] = count(< n)
        let mut cumulative_count_vec = generate_cumulative_count_vec(&count_vec);

        for idx in 0..vec.len() {
            let mut isize_cur_elem: isize = vec[idx].try_into().unwrap();
            let mut shifted_elem_val = isize_cur_elem.abs_sub(&isize_min) as usize;

            // The correct index of the current element
            let mut correct_idx = &mut cumulative_count_vec[shifted_elem_val];

            // If the current element is in the wrong spot, rotate an entire cycle once
            if idx >= *correct_idx {
                let mut cur_elem = vec[idx];
                
                loop {
                    isize_cur_elem = cur_elem.try_into().unwrap();
                    shifted_elem_val = isize_cur_elem.abs_sub(&isize_min) as usize;
                    correct_idx = &mut cumulative_count_vec[shifted_elem_val];

                    // Swap with current element with the one at it's correct index
                    swap(&mut cur_elem, &mut vec[*correct_idx]);

                    // Increment the correct index for a potential next element
                    *correct_idx += 1;
                    
                    // Exit condition: we've rotated around the entire cycle once
                    if *correct_idx - 1 == idx {
                        break;
                    }
                }
            }
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Unstable Ranged Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("URC");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}