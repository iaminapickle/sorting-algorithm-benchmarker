use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

// Stable sorting algorithm using rust's default
pub struct StableRustSort {}

impl Default for StableRustSort {
    fn default() -> StableRustSort {
        return StableRustSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for StableRustSort {
    fn sort(&self, vec: &mut [T]) {
        vec.sort();
    }

    fn name(&self) -> String {
        return String::from("Stable Rust Sort");
    }

    fn nickname(&self) -> String{
        return String::from("SR");
    }

    fn is_stable(&self) -> bool {
        return true;
    }
}
