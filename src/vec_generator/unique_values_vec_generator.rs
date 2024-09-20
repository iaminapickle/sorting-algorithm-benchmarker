use rand::Rng;
use rand::seq::SliceRandom;
use crate::vec_generator::VecGenerator;

// Generates a vector of size { size } consisting of { var } guaranteed unique values between { lower_bound } and { upper_bound } 
pub struct UniqueValueVecGenerator {
    pub size: usize,
    pub lower_bound: i128,
    pub upper_bound: i128,
}

impl VecGenerator<i128> for UniqueValueVecGenerator {
    fn generate(&self, var: i128) -> Vec<i128> {
        let mut unique_values: Vec<i128> = Vec::with_capacity(usize::try_from(var).unwrap());
        
        let mut rng = rand::thread_rng();
        for _ in 0..var {
            // Generate random value in range
            let val = rng.gen_range(self.lower_bound..=self.upper_bound);

            unique_values.push(val);
        }

        let mut vec: Vec<i128> = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            let val = *unique_values.choose(&mut rng).unwrap();
        
            vec.push(val);
        }
        return vec;
    }
}