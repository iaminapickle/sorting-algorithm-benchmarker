use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct BubbleSort {}

impl Default for BubbleSort {
    fn default() -> BubbleSort {
        return BubbleSort {};
    }
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for BubbleSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut num_sorted = 0;
        let mut swapped = true;
        
        while swapped == true {
            swapped = false;

            // Float a minimum value to the right
            for i in 0..vec.len() - num_sorted - 1 {
                if vec[i] > vec[i + 1] {
                    swapped = true;
                    vec.swap(i, i + 1);
                }
            }
            num_sorted += 1;
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Bubble Sort");
    }

    fn nickname(&self) -> String {
        return String::from("B");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}