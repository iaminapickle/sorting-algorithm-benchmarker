use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::rust_sort::stable_rust_sort::StableRustSort;
use crate::sorting_algorithm::rust_sort::unstable_rust_sort::UnstableRustSort;


sorting_tests! {
    stable_rust_sort: StableRustSort::default(),
    unstable_rust_sort: UnstableRustSort::default(),
}