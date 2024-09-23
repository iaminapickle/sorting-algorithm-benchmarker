use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::selection_sort::stable_bidirectional_selection_sort::StableBidirectionalSelectionSort;
use crate::sorting_algorithm::selection_sort::stable_grouped_selection_sort::StableGroupedSelectionSort;
use crate::sorting_algorithm::selection_sort::stable_selection_sort::StableSelectionSort;
use crate::sorting_algorithm::selection_sort::unstable_bidirectional_selection_sort::UnstableBidirectionalSelectionSort;
use crate::sorting_algorithm::selection_sort::unstable_grouped_selection_sort::UnstableGroupedSelectionSort;
use crate::sorting_algorithm::selection_sort::unstable_selection_sort::UnstableSelectionSort;

sorting_tests! {
    stable_bidirectional_selection_sort: StableBidirectionalSelectionSort::default(),
    stable_grouped_selection_sort: StableGroupedSelectionSort::default(),
    stable_selection_sort: StableSelectionSort::default(),
    unstable_bidirectional_selection_sort: UnstableBidirectionalSelectionSort::default(),
    unstable_grouped_selection_sort: UnstableGroupedSelectionSort::default(),
    unstable_selection_sort: UnstableSelectionSort::default(),
}