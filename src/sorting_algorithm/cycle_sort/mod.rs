pub mod unstable_general_cycle_sort;
pub mod stable_general_cycle_sort;

pub mod unstable_ranged_cycle_sort;
pub mod stable_ranged_cycle_sort;

pub mod unstable_wiki_cycle_sort;
pub mod stable_wiki_cycle_sort;

pub mod special_cycle_sort;

#[cfg(test)]
pub mod tests;

use std::collections::BTreeMap;
use std::fmt::Debug;
use num::Signed;

// Generates an BTreeMap from a slice such that map[key] = count(key)
fn generate_count_map<T: Ord + Copy + Debug>(vec: &[T]) -> BTreeMap<T, usize> {
    let mut count_map: BTreeMap<T, usize> = BTreeMap::new();

    for elem in vec {
        *count_map.entry(*elem).or_insert(0) += 1;
    }

    return count_map;
}

// Generates a BTreeMap from a count_map such that map[key] = count(< key)
fn generate_cumulative_count_map<T: Ord + Copy + Debug>(count_map: &BTreeMap<T, usize>) -> BTreeMap<T, usize> {
    if count_map.len() == 0 { return BTreeMap::new(); }

    let first_key = *count_map.keys().next().unwrap();
    let mut cumulative_count_map: BTreeMap<T, usize> = BTreeMap::from([
        (first_key, 0)
    ]);

    let mut prev: usize = *count_map.values().next().unwrap();
    for key in count_map.keys().skip(1) {
        cumulative_count_map.insert(*key, prev);
        
        prev += *count_map.get(key).unwrap();
    }  
    return cumulative_count_map;
}

// Generates a Vec from a slice such that vec[n] = count(key)
fn generate_count_vec<T: Ord + Copy + Debug> (vec: &[T], min: T, max: T) -> Vec<usize> where 
    T: TryInto<isize>, <T as TryInto<isize>>::Error: Debug
{
    // Normalise the range such that 0 is the minimum
    let isize_max: isize = max.try_into().unwrap();
    let isize_min: isize = min.try_into().unwrap();
    let shifted_max = isize_max.abs_sub(&isize_min) as usize;

    let mut count_vec: Vec<usize> = vec![0; shifted_max + 1];

    for elem in vec {
        let isize_elem: isize = (*elem).try_into().unwrap();

        let shifted_elem_val = isize_elem.abs_sub(&isize_min) as usize;
        count_vec[shifted_elem_val] += 1;
    }

    return count_vec;
}

// Generates a Vec from a slice such that vec[n] = count(< n);
fn generate_cumulative_count_vec(count_vec: &Vec<usize>) -> Vec<usize> {
    if count_vec.len() == 0 { return Vec::new(); }

    let mut cumulative_count_vec : Vec<usize> = vec![0; count_vec.len()];
    
    for (idx, _) in count_vec[1..].iter().enumerate() {
        cumulative_count_vec[idx + 1] = cumulative_count_vec[idx] + count_vec[idx];
    }

    return cumulative_count_vec;
}

