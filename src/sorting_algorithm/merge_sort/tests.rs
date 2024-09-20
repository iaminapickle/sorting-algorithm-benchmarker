use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::{MergeSort, SortingAlgorithm, SortElem, to_sort_elem_vec};
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sorting_algorithm::run_sort_in_thread;

sorting_tests! {
    merge_sort: MergeSort::default(),
}