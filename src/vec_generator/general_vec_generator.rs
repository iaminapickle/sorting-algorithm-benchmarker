use rand::Rng;
use crate::vec_generator::VecGenerator;

// Generates a vector of size { var } with values between { lower_bound } and { upper_bound } - both inclusive
pub struct GeneralVecGenerator {
    pub lower_bound: i128,
    pub upper_bound: i128,
}

impl VecGenerator<i128> for GeneralVecGenerator {
    fn generate(&self, var: i128) -> Vec<i128> {
        let mut vec: Vec<i128> = Vec::with_capacity(usize::try_from(var).ok().unwrap());
        
        let mut rng = rand::thread_rng();
        for _ in 0..var {
            // Generate random value in range
            let val = rng.gen_range(self.lower_bound..=self.upper_bound);

            vec.push(val);
        }
        return vec;
    }
}