use std::fmt::Debug;
use crate::sorting_algorithm::SortingAlgorithm;

// Unstable sorting algorithm using rust's default
pub struct UnstableRustSort {}

impl Default for UnstableRustSort {
    fn default() -> UnstableRustSort {
        return UnstableRustSort {};
    }
}

impl<T: Ord + Copy + Debug> SortingAlgorithm<T> for UnstableRustSort {
    fn sort(&self, vec: &mut [T]) {
        vec.sort_unstable();
    }

    fn name(&self) -> String {
        return String::from("Unstable Rust Sort");
    }

    fn nickname(&self) -> String{
        return String::from("UR");
    }

    fn is_stable(&self) -> bool {
        return false;
    }
}