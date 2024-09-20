use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::{StableBidirectionalSelectionSort,   StableGroupedSelectionSort,   StableSelectionSort,
            UnstableBidirectionalSelectionSort, UnstableGroupedSelectionSort, UnstableSelectionSort,
            SortingAlgorithm, SortElem, to_sort_elem_vec};
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sorting_algorithm::run_sort_in_thread;

sorting_tests! {
    stable_bidirectional_selection_sort: StableBidirectionalSelectionSort::default(),
    stable_grouped_selection_sort: StableGroupedSelectionSort::default(),
    stable_selection_sort: StableSelectionSort::default(),
    unstable_bidirectional_selection_sort: UnstableBidirectionalSelectionSort::default(),
    unstable_grouped_selection_sort: UnstableGroupedSelectionSort::default(),
    unstable_selection_sort: UnstableSelectionSort::default(),
}