use std::fmt::Debug;
use std::mem::swap;
use std::hash::Hash;
use num::Signed;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::sort_elem::{SortElem, to_sort_elem_vec};
use crate::helper::find_min_max_in_slice;
use crate::sorting_algorithm::cycle_sort::{generate_count_vec, generate_cumulative_count_vec};

pub struct StableRangedCycleSort {}

impl Default for StableRangedCycleSort {
    fn default() -> StableRangedCycleSort {
        return StableRangedCycleSort {};
    }
}

impl<T: Ord + Copy + Debug + Hash> SortingAlgorithm<T> for StableRangedCycleSort where 
    T: TryInto<isize>, <T as TryInto<isize>>::Error: Debug
{
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let ((_, min_elem), (_, max_elem)) = find_min_max_in_slice(vec).unwrap();
        let count_vec = generate_count_vec(vec, min_elem, max_elem);

        // Minimum value as isize
        let isize_min: isize = min_elem.try_into().unwrap();

        // Generate the cumulative count Vec
        // This is a vector such that vec[n] = count(< n)
        let cumulative_count_vec = generate_cumulative_count_vec(&count_vec);

        // Convert the vector to a SortElemVec
        let mut sort_elem_vec = to_sort_elem_vec(vec.to_vec());

        for idx in 0..sort_elem_vec.len() {
            let mut cur_sort_elem: SortElem<T> = sort_elem_vec[idx];
            
            loop {
                // Current element as isize
                let isize_cur_sort_elem: isize = cur_sort_elem.val.try_into().unwrap();

                // Current element shifted
                let shifted_elem_val = isize_cur_sort_elem.abs_sub(&isize_min) as usize;
                
                // The correct index of the current element
                let correct_idx = cumulative_count_vec[shifted_elem_val] + cur_sort_elem.occ;
                
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
        return String::from("Stable Ranged Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SRC");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}