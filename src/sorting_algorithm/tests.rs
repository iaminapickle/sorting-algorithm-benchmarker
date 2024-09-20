#[macro_export]
macro_rules! sorting_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        paste! {
            #[test]
            fn [<$name _empty_test>]() {
                let mut vec = vec![];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _already_sorted_test>]() {
                let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _reverse_sorted_test>]() {
                let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _all_duplicate_test>]() {
                let mut vec = vec![0, 0, 0, 0, 0];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _odd_len_test>]() {
                let mut vec = vec![3, 1, 2, 0, 5, 4, 10];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _even_len_test>]() {
                let mut vec = vec![3, 1, 2, 0, 5, 10];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _singleton_test>]() {
                let mut vec = vec![0];
                let algo: Box<dyn SortingAlgorithm<usize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _isize_test>]() {
                let mut vec = vec![-1, 0, -2, 1];
                let algo: Box<dyn SortingAlgorithm<isize> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                assert_eq!(is_unstable_sorted(&vec), true);
            }

            #[test]
            fn [<$name _sort_elem_test>]() {
                let mut vec = to_sort_elem_vec(vec![-1, 0, -2, 1]);
                let algo: Box<dyn SortingAlgorithm<SortElem<isize>> + Sync> = Box::new($value);

                run_sort_in_thread(&algo, &mut vec, Duration::from_secs(10));

                let res = if algo.is_stable() { is_stable_sorted(&vec) } else { is_unstable_sorted(&vec) }; 
                assert_eq!(res, true);
            }
        }
    )*
    }
}