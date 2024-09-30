extern crate sorting_algorithm_benchmarker;

use sorting_algorithm_benchmarker::*;

use crate::sorting_algorithm_benchmarker::sorting_algorithm::SortingAlgorithm;
use crate::sorting_algorithm::quick_sort::stable_three_way_quick_sort::StableThreeWayQuickSort;

fn main() {
    let algo = StableThreeWayQuickSort::default();
    let mut v = vec![4, 3, 3, 2];

    algo.sort(&mut v);

    println!("Res: {:?}", v);
}   