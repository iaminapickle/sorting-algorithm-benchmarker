use std::fmt::Debug;
use std::mem::swap;
use num::Signed;
use crate::sorting_algorithm::SortingAlgorithm;
use crate::helper::find_min_in_slice;

pub struct SpecialCycleSort {}

impl Default for SpecialCycleSort {
    fn default() -> SpecialCycleSort {
        return SpecialCycleSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for SpecialCycleSort where
    T: TryInto<isize>, <T as TryInto<isize>>::Error: Debug
{
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let (_, min_elem) = find_min_in_slice(vec).unwrap();
        let isize_min: isize = min_elem.try_into().unwrap();

        for idx in 0..vec.len() {
            let mut cur_elem = vec[idx];
            loop {
                let isize_cur: isize = cur_elem.try_into().unwrap();
                let correct_idx = isize_cur.abs_sub(&isize_min) as usize;
                swap(&mut cur_elem, &mut vec[correct_idx]);
                
                if correct_idx == idx {
                    break;
                }

            }
        }
        return;
    }

    fn name(&self) -> String {
        return String::from("Special Cycle Sort");
    }

    fn nickname(&self) -> String {
        return String::from("SC");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}