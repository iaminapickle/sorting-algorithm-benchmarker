pub mod selection_sort;
pub mod insertion_sort;
pub mod cycle_sort;
pub mod bubble_sort;
pub mod merge_sort;
pub mod rust_sort;
pub mod quick_sort;

#[cfg(test)]
pub mod tests;

use std::fmt::Debug;
use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

pub trait SortingAlgorithm<T: Ord + Copy + Debug> {
    fn sort(&self, vec: &mut [T]);
    fn name(&self) -> String;
    fn nickname(&self) -> String;
    fn is_stable(&self) -> bool;
}

// Starts the sort in a thread with a specified timeout
// Returns None if the test failed or timed out
pub fn run_sort_in_thread<T: Ord + Copy + Debug + Send>(
    algorithm: &Box<dyn SortingAlgorithm<T> + Sync>, 
    mut vec: &mut Vec<T>, 
    time_limit: Duration) -> Option<Duration>
{
    let (sender, receiver) = mpsc::channel();
    let mut res: Option<Duration> = None;
    thread::scope(|scope| {
        scope.spawn(move || {
            match sender.send({
                let start = Instant::now();
                algorithm.sort(&mut vec);
                start.elapsed()
            }) {
                Ok(_) => {},
                Err(_) => {},
            }
        });

        res = match receiver.recv_timeout(time_limit) {
            Ok(n) => Some(n), 
            Err(_) => None,
        };
    });
    return res;
}