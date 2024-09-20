use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct InsertionSort {}

impl Default for InsertionSort {
    fn default() -> InsertionSort {
        return InsertionSort {};
    }
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for InsertionSort {
    fn sort(&self, vec: &mut [T]) {
        for i in 0..vec.len() {
            let cur = vec[i];
            let mut j = i;

            // Shift all elements larger than the current value to the right by one
            while j > 0 && vec[j - 1] > cur {
                vec[j] = vec[j - 1];
                j-= 1;
            }

            vec[j] = cur;
        }

        return;
    }

    fn name(&self) -> String {
        return String::from("Insertion Sort");
    }

    fn nickname(&self) -> String {
        return String::from("I");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}