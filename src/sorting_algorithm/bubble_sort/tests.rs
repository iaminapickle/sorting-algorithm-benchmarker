use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::bubble_sort::bidirectional_bubble_sort::BidirectionalBubbleSort;
use crate::sorting_algorithm::bubble_sort::bubble_sort::BubbleSort;

sorting_tests! {
    bubble_sort: BubbleSort::default(),
    bidirectional_bubble_sort: BidirectionalBubbleSort::default(),
}