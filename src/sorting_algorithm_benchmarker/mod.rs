use std::hash::Hash;
use std::fmt::{Debug, Formatter, Result};
use std::time::Duration;
use std::cmp::max;

use colored::Colorize;

use crate::sorting_algorithm::{SortingAlgorithm, run_sort_in_thread};
use crate::sort_elem::{SortElem, to_sort_elem_vec};
use crate::helper::{is_unstable_sorted, is_stable_sorted};
use crate::vec_generator::VecGenerator;

#[derive(Clone, Copy)]
pub enum DurationOptions {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

fn format_duration_suffixed(dur: Duration, opt: DurationOptions) -> String {
    return format_duration(dur, opt) + &duration_suffix(opt);
}

fn duration_suffix(opt: DurationOptions) -> String {
    return match opt {
        DurationOptions::Second => "s".to_string(),
        DurationOptions::Millisecond => "ms".to_string(),
        DurationOptions::Microsecond => "μs".to_string(),
        DurationOptions::Nanosecond => "ns".to_string(),
    };
}

fn format_duration(dur: Duration, opt: DurationOptions) -> String {
    return match opt {
        DurationOptions::Second => dur.as_secs().to_string(),
        DurationOptions::Millisecond => dur.as_millis().to_string(),
        DurationOptions::Microsecond => dur.as_micros().to_string(),
        DurationOptions::Nanosecond => dur.as_nanos().to_string(),
    };
}

const NO_PASSED_BENCHMARK_STRING: &str = "NaN";

// Wrapper struct for all time test results
pub struct BenchmarkResults {
    // X axis of grid -> generator args
    generator_args: Vec<i128>,

    // Y axis of grid -> names
    algorithm_names: Vec<String>,

    // 2D Vec representing the benchmark results for each algorithm and generator argument
    // Indexed by generator argument then algorithm both according to their respective indices
    benchmark_times: Vec<Vec<Duration>>,

    // 2d Vec representing the number of benchmarks passed for each algorithm and generator argument
    // Note: Algorithms that pass 0 benchmarks are printed according to NO_PASSED_BENCHMARK_STRING
    no_benchmarks_passed: Vec<Vec<usize>>,

    // Stores the length of the longest string in each column
    // Used for pretty printing
    longest_column_strings: Vec<usize>,

    // Setting for printing durations
    duration_option: DurationOptions,

    // Spacing between columns
    column_offset: usize,
}

pub struct SortingAlgorithmBenchmarker
{  
    // Number of benchmarks to average upon
    pub no_benchmarks: u128,

    // Boolean that determines whether or not to use nicknames when printing
    pub use_nicknames: bool,

    // Boolean for printing incorrectly sorted vectors
    pub print_incorrect_vectors: bool,

    // Boolean on whether to stop immediately upon a failed sort or not
    pub stop_on_fail: bool,

    // Time limit before the benchmark is considered a failure
    pub benchmark_time_limit: Duration,

    // Option for printing durations
    pub duration_option: DurationOptions 
}

impl SortingAlgorithmBenchmarker where
{
    pub fn run_benchmarks<T,G> (&self,
                algorithms: Vec<Box<dyn SortingAlgorithm<SortElem<T>> + Sync>>,
                generator: G,
                generator_args: Vec<i128>
            ) -> Option<BenchmarkResults> where 
        T: Ord + Copy + Debug + Hash + Send,
        G: VecGenerator<T>, 
    {
        if algorithms.len() == 0 || generator_args.len() == 0 { return None; }
        
        let algorithm_names: Vec<String> = if self.use_nicknames { algorithms.iter().map(|algo| algo.nickname()).collect::<Vec<String>>() } 
                                           else { algorithms.iter().map(|algo| algo.name()).collect::<Vec<String>>() };
        // Longest algorithm name OR ({duration_suffix})
        let longest_algorithm_name: usize = max(*algorithm_names.iter().map(|name| name.len()).collect::<Vec<usize>>().iter().max().unwrap(),
                                                duration_suffix(self.duration_option).len() + 2);

        let mut benchmark_times: Vec<Vec<Duration>> = Vec::new();
        let mut no_benchmarks_passed: Vec<Vec<usize>> = Vec::new();
        let mut longest_column_strings: Vec<usize> = Vec::with_capacity(generator_args.len() + 1);
        longest_column_strings.push(longest_algorithm_name);

        // For each generator argument
        for (_, n) in generator_args.iter().enumerate() {
            let mut res_times: Vec<Duration> = vec![Duration::default(); algorithms.len()];
            let mut res_no_passed: Vec<usize> = vec![0; algorithms.len()];
            
            // Run a specified number of benchmarks for each algorithm and store the result
            for _ in 0..self.no_benchmarks {
                let vec = to_sort_elem_vec(generator.generate(*n));

                println!("With arg {}:", n);
                for (algo_idx, algorithm) in algorithms.iter().enumerate() {
                    let mut vec_clone = vec.clone();
                    let formatted_algo_name = algorithm_names[algo_idx].clone() + &" ".repeat(longest_algorithm_name - &algorithm_names[algo_idx].len());

                    match run_sort_in_thread(algorithm, &mut vec_clone, self.benchmark_time_limit) {
                        // Sorting function finished and returned 
                        Some(dur) => {
                            // Sometimes the time limit isn't honoured 
                            if dur > self.benchmark_time_limit {
                                println!("{} {}: timed out (> {})", formatted_algo_name, "FAILED".red(), format_duration_suffixed(self.benchmark_time_limit, self.duration_option));
                                break;
                            }
                            let correct = if algorithm.is_stable() { is_stable_sorted(&vec_clone) } else { is_unstable_sorted(&vec_clone) };
                            
                            // Correct sort
                            if correct {
                                res_times[algo_idx] += dur;
                                res_no_passed[algo_idx] += 1;
                                println!("{} {}: took {}", formatted_algo_name, "PASSED".green(), format_duration_suffixed(dur, self.duration_option));
                            }
                            // Incorrect sort
                            else {
                                println!("{} {}:\nInput:  {:?}\nOutput: {:?}", formatted_algo_name, "FAILED".red(), vec, vec_clone);
                            }
                        }

                        // Sorting function took longer than the specified time
                        None => {
                            println!("{} {}: timed out (> {})", formatted_algo_name, "FAILED".red(), format_duration_suffixed(self.benchmark_time_limit, self.duration_option));
                        }
                    }
                }
                println!("");
            }
            
            let mut longest_column_string = n.to_string().len();
            for idx in 0..res_times.len() {
                let no_passed = res_no_passed[idx];

                // Average times with the number of tests passed
                if no_passed != 0 {
                    res_times[idx] /= no_passed.try_into().unwrap();
                    longest_column_string = max(longest_column_string, format_duration(res_times[idx], self.duration_option).to_string().len());
                } else {
                    longest_column_string = max(longest_column_string, NO_PASSED_BENCHMARK_STRING.len());
                }
            }

            benchmark_times.push(res_times);
            no_benchmarks_passed.push(res_no_passed);
            longest_column_strings.push(longest_column_string);
        }

        return Some(BenchmarkResults {
            generator_args,
            algorithm_names,
            benchmark_times,
            no_benchmarks_passed,
            longest_column_strings,
            duration_option: self.duration_option,
            column_offset : 2,
        })
    }
}

impl Debug for BenchmarkResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Empty top left
        write!(f, "({}){}",
            duration_suffix(self.duration_option),
            " ".repeat(self.longest_column_strings[0] - duration_suffix(self.duration_option).len() - 2 + self.column_offset)
        )?;

        // Print all generator arguments
        for (idx, arg) in self.generator_args.iter().enumerate() {
            write!(f, "{}{}", 
                arg.to_string(),
                " ".repeat(self.longest_column_strings[idx + 1] - arg.to_string().len() + self.column_offset)
            )?;
        }
        write!(f, "\n")?;

        // Print result rows
        for (i, algo_name) in self.algorithm_names.iter().enumerate() {
            // Print algorithm name
            write!(f, "{}{}",
                algo_name,
                " ".repeat(self.longest_column_strings[0] - algo_name.len() + self.column_offset)
            )?;

            // Print benchmark result for each algorithm
            for (j, times) in self.benchmark_times.iter().enumerate() {
                let res = if self.no_benchmarks_passed[j][i] == 0 { NO_PASSED_BENCHMARK_STRING.to_string() } else { format_duration(times[i], self.duration_option) };
                write!(f, "{}{}",
                    res.to_string(),
                    " ".repeat(self.longest_column_strings[j + 1] - res.len() + self.column_offset)
                )?;
            }
            write!(f, "\n")?;
        }

        return Ok(());
    }
}