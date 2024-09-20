use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

pub struct BidirectionalBubbleSort {}

impl Default for BidirectionalBubbleSort {
    fn default() -> BidirectionalBubbleSort {
        return BidirectionalBubbleSort {};
    }
}

impl<T:Ord + Copy + Debug> SortingAlgorithm<T> for BidirectionalBubbleSort {
    fn sort(&self, vec: &mut [T]) {
        if vec.len() == 0 { return; }

        let mut num_sorted = 0;
        let mut swapped = true;

        while swapped == true {
            swapped = false;
            // Float a minimum value to the right
            for i in 0..vec.len() - num_sorted - 1 {
                if vec[i] > vec[i + 1] {
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            }

            // If no swap happened, the array is already sorted
            if swapped == false { break; }

            // Sink a maximum value to the left
            for i in vec.len() - num_sorted - 1 .. 0 {
                if vec[i] < vec[i - 1] {
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            }

            // If no swap happened, the array is already sorted
            if swapped == false { break; }

            num_sorted += 1;
        }
        return;
    }

    fn name(&self) -> String {
        return String::from("Bidirectional Bubble Sort");
    }

    fn nickname(&self) -> String {
        return String::from("BB");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}