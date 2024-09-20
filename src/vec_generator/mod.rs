pub mod general_vec_generator;
pub mod unique_values_vec_generator;

use std::fmt::Debug;

pub trait VecGenerator<T: Ord + Copy + Debug> {
    fn generate(&self, var: i128) -> Vec<T>;
}