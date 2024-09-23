use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::insertion_sort::insertion_sort::InsertionSort;
use crate::sorting_algorithm::insertion_sort::binary_insertion_sort::BinaryInsertionSort;

sorting_tests! {
    insertion_sort: InsertionSort::default(),
    binary_insertion_sort: BinaryInsertionSort::default(),
}