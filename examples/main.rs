extern crate sorting_algorithm_benchmarker;
use std::time::Duration;

use sorting_algorithm_benchmarker::*;

use crate::sort_elem::SortElem;

use crate::vec_generator::general_vec_generator::GeneralVecGenerator;

use crate::sorting_algorithm::SortingAlgorithm;
use crate::sorting_algorithm::selection_sort::unstable_selection_sort::UnstableSelectionSort;
use crate::sorting_algorithm::selection_sort::stable_selection_sort::StableSelectionSort;

use crate::sorting_algorithm::selection_sort::unstable_bidirectional_selection_sort::UnstableBidirectionalSelectionSort;
use crate::sorting_algorithm::selection_sort::stable_bidirectional_selection_sort::StableBidirectionalSelectionSort;

use crate::sorting_algorithm::selection_sort::stable_grouped_selection_sort::StableGroupedSelectionSort;
use crate::sorting_algorithm::selection_sort::unstable_grouped_selection_sort::UnstableGroupedSelectionSort;

use crate::sorting_algorithm::cycle_sort::unstable_general_cycle_sort::UnstableGeneralCycleSort;
use crate::sorting_algorithm::cycle_sort::stable_general_cycle_sort::StableGeneralCycleSort;

use crate::sorting_algorithm::cycle_sort::unstable_ranged_cycle_sort::UnstableRangedCycleSort;
use crate::sorting_algorithm::cycle_sort::stable_ranged_cycle_sort::StableRangedCycleSort;

use crate::sorting_algorithm::cycle_sort::unstable_wiki_cycle_sort::UnstableWikiCycleSort;
use crate::sorting_algorithm::cycle_sort::stable_wiki_cycle_sort::StableWikiCycleSort;

use crate::sorting_algorithm::rust_sort::unstable_rust_sort::UnstableRustSort;
use crate::sorting_algorithm::rust_sort::stable_rust_sort::StableRustSort;

use crate::sorting_algorithm::bubble_sort::bubble_sort::BubbleSort;
use crate::sorting_algorithm::bubble_sort::bidirectional_bubble_sort::BidirectionalBubbleSort;

use crate::sorting_algorithm::insertion_sort::insertion_sort::InsertionSort;
use crate::sorting_algorithm::insertion_sort::binary_insertion_sort::BinaryInsertionSort;

use crate::sorting_algorithm::merge_sort::merge_sort::MergeSort;

use crate::sorting_algorithm::quick_sort::quick_sort::QuickSort;
use crate::sorting_algorithm::quick_sort::unstable_three_way_quick_sort::UnstableThreeWayQuickSort;
use crate::sorting_algorithm::quick_sort::stable_three_way_quick_sort::StableThreeWayQuickSort;

use crate::sorting_algorithm_benchmarker::sorting_algorithm_benchmarker::{SortingAlgorithmBenchmarker, DurationOptions};

fn main() {
    // Default Rust Sort Algorithms
    let stable_rust_sort = StableRustSort::default();
    let unstable_rust_sort = UnstableRustSort::default();
    
    // Selection Sort Algorithms
    let unstable_selection_sort = UnstableSelectionSort::default();
    let stable_selection_sort = StableSelectionSort::default();

    let unstable_bidirectional_selection_sort = UnstableBidirectionalSelectionSort::default();
    let stable_bidirectional_selection_sort = StableBidirectionalSelectionSort::default();

    let unstable_grouped_selection_sort = UnstableGroupedSelectionSort::default();
    let stable_grouped_selection_sort = StableGroupedSelectionSort::default();
    
    // Cycle Sort Algorithms
    let unstable_general_cycle_sort = UnstableGeneralCycleSort::default();
    let stable_general_cycle_sort = StableGeneralCycleSort::default();

    let unstable_ranged_cycle_sort = UnstableRangedCycleSort::default();
    let stable_ranged_cycle_sort = StableRangedCycleSort::default();

    let unstable_wiki_cycle_sort = UnstableWikiCycleSort::default();
    let stable_wiki_cycle_sort = StableWikiCycleSort::default();

    let bubble_sort = BubbleSort::default();
    let bidirectional_bubble_sort = BidirectionalBubbleSort::default();

    let insertion_sort = InsertionSort::default();
    let binary_insertion_sort = BinaryInsertionSort::default();

    let merge_sort = MergeSort::default();

    let quick_sort = QuickSort::default();
    let unstable_three_way_quick_sort = UnstableThreeWayQuickSort::default();
    let stable_three_way_quick_sort = StableThreeWayQuickSort::default();

    let generator = GeneralVecGenerator {
        lower_bound: 0,
        upper_bound: 1000,
    };

    let algorithms: Vec<Box<dyn SortingAlgorithm<SortElem<i128>> + Sync>> = vec![
        Box::new(unstable_rust_sort), Box::new(stable_rust_sort),
        Box::new(bubble_sort), Box::new(bidirectional_bubble_sort),
        Box::new(unstable_selection_sort), Box::new(stable_selection_sort),
        Box::new(unstable_bidirectional_selection_sort), Box::new(stable_bidirectional_selection_sort),
        Box::new(unstable_grouped_selection_sort), Box::new(stable_grouped_selection_sort),
        Box::new(insertion_sort), Box::new(binary_insertion_sort), 
        Box::new(unstable_general_cycle_sort), Box::new(stable_general_cycle_sort),
        Box::new(unstable_ranged_cycle_sort),  Box::new(stable_ranged_cycle_sort),
        Box::new(unstable_wiki_cycle_sort), Box::new(stable_wiki_cycle_sort),
        Box::new(merge_sort),
        Box::new(quick_sort),
        Box::new(unstable_three_way_quick_sort), Box::new(stable_three_way_quick_sort),
    ];
    
    let range = vec![999, 1000, 9999, 10000];

    let benchmarker = SortingAlgorithmBenchmarker {
        no_benchmarks: 3,
        use_nicknames: false,
        print_incorrect_vectors: true,
        stop_on_fail: false,
        benchmark_time_limit: Duration::from_secs(10),
        duration_option: DurationOptions::Millisecond,
    };

    let res = benchmarker.run_benchmarks(algorithms, generator, range).unwrap();
    println!("{:?}", res);
}   