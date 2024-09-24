use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::quick_sort::quick_sort::QuickSort;
use crate::sorting_algorithm::quick_sort::three_way_quick_sort::ThreeWayQuickSort;

sorting_tests! {
    quick_sort: QuickSort::default(),
    three_way_quick_sort: ThreeWayQuickSort::default(),
}