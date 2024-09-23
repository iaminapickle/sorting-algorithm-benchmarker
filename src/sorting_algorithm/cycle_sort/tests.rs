use paste::paste;
use std::time::Duration;

use crate::sorting_tests;
use crate::helper::{is_stable_sorted, is_unstable_sorted};
use crate::sort_elem::{to_sort_elem_vec, SortElem};

use crate::sorting_algorithm::{run_sort_in_thread, SortingAlgorithm};
use crate::sorting_algorithm::cycle_sort::stable_general_cycle_sort::StableGeneralCycleSort;
use crate::sorting_algorithm::cycle_sort::stable_ranged_cycle_sort::StableRangedCycleSort;
use crate::sorting_algorithm::cycle_sort::stable_wiki_cycle_sort::StableWikiCycleSort;
use crate::sorting_algorithm::cycle_sort::unstable_general_cycle_sort::UnstableGeneralCycleSort;
use crate::sorting_algorithm::cycle_sort::unstable_ranged_cycle_sort::UnstableRangedCycleSort;
use crate::sorting_algorithm::cycle_sort::unstable_wiki_cycle_sort::UnstableWikiCycleSort;
use crate::sorting_algorithm::cycle_sort::special_cycle_sort::SpecialCycleSort;

sorting_tests! {
    stable_general_cycle_sort: StableGeneralCycleSort::default(),
    stable_ranged_cycle_sort: StableRangedCycleSort::default(),
    stable_wiki_cycle_sort: StableWikiCycleSort::default(),
    unstable_general_cycle_sort: UnstableGeneralCycleSort::default(),
    unstable_ranged_cycle_sort: UnstableRangedCycleSort::default(),
    unstable_wiki_cycle_sort: UnstableWikiCycleSort::default(),
}

#[test]
fn special_cycle_sort_empty_test() {
    let mut vec = vec![];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}

#[test]
fn special_cycle_sort_already_sorted_test() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}

#[test]
fn special_cycle_sort_reverse_sorted_test() {
    let mut vec = vec![5, 4, 3, 2, 1, 0];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}

#[test]
fn special_cycle_sort_odd_len_test() {
    let mut vec = vec![4, 2, 1, 5, 3];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}

#[test]
fn special_cycle_sort_even_len_test() {
    let mut vec = vec![4, 2, 1, 3];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}

#[test]
fn special_cycle_sort_singleton_test() {
    let mut vec = vec![0];
    let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new(SpecialCycleSort::default());

    run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

    assert_eq!(is_unstable_sorted(&vec), true);
}