use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use crate::sort_elem::SortElem;

// Checks if a vector is sorted without regards to stability
pub fn is_unstable_sorted<T: Ord + Copy + Debug>(vec: &[T]) -> bool {
    return vec.windows(2).all(|w| w[0] <= w[1]);
}

// Checks if a vector is sort with stability
pub fn is_stable_sorted<T: Ord + Copy + Debug + Hash + Eq>(vec: &[SortElem<T>]) -> bool {
    let mut occ_map: HashMap<T, usize> = HashMap::new();
    for w in vec.windows(2) {
        if !occ_map.contains_key(&w[0].val) {
            occ_map.insert(w[0].val, 0);
        } else {
            occ_map.entry(w[0].val).and_modify(|n| *n += 1);
        }

        if w[0].val > w[1].val || occ_map[&w[0].val] != w[0].occ {
            return false;
        } 
    }

    return true;
}

// Returns a Vec of the minimum values (index: value)
// The indices are NOT absolute but relative to the slice
pub fn find_all_min_in_slice<T: Ord + Copy + Debug>(vec: &[T]) -> 
        Option<(Vec<(usize, T)>, T)>{
    if vec.len() == 0 {
        return None;
    }

    let min_val = find_min_in_slice(vec).unwrap().1;

    // Vec tracking minimum (idx: val)
    let mut all_min_vec = vec![];

    for (idx, elem) in vec.iter().enumerate() {
        if *elem == min_val {
            all_min_vec.push((idx, *elem));
        }
    }

    return Some((all_min_vec, min_val));
}

// Returns a tuple of the minimum integer (index, value) 
// The indices are NOT absolute but relative to the slice
pub fn find_min_in_slice<T: Ord + Copy + Debug>(vec: &[T]) -> Option<(usize, T)> {
    if vec.len() == 0 {
        return None;
    }
    
    let mut min = (0, vec[0]);
    for (idx, elem) in vec[1..].iter().enumerate() {
        if *elem < min.1 {
            min = (idx + 1, *elem);
        }
    }

    return Some(min);
}

// Returns a tuple of the minimum value and the maximum value ((index, value), (index, value))
// The indices are NOT absolute but relative to the slice
pub fn find_min_max_in_slice<T: Ord + Copy + Debug>(vec: &[T]) -> 
        Option<((usize, T), (usize, T))> {
    if vec.len() == 0 {
        return None;
    }

    if vec.len() == 1 {
        return Some(((0, vec[0]), (0, vec[0])));
    }

    let mut cur_min: (usize, T);
    let mut cur_max: (usize, T);

    if vec[0] <= vec[1] {
        cur_min = (0, vec[0]);
        cur_max = (1, vec[1]);
    } else {
        cur_min = (1, vec[1]);
        cur_max = (0, vec[0]);
    }

    // Consider every two elements in pairs
    for (idx, pair) in vec[2..].chunks_exact(2).enumerate() {
        let a = pair[0];
        let b = pair[1];
        let smaller: (usize, T);
        let larger: (usize, T);

        if a <= b {
            smaller = ((idx + 1) * 2, a);
            larger = ((idx + 1) * 2 + 1, b);
        } else {
            smaller = ((idx + 1) * 2 + 1, b);
            larger = ((idx + 1) * 2, a);
        }

        if smaller.1 < cur_min.1 {
            cur_min = smaller;
        }

        if larger.1 >= cur_max.1 {
            cur_max = larger;
        }
    }

    // Last element in odd length vector
    if vec.len() % 2 == 1 {
        if *vec.last().unwrap() >= cur_max.1 {
            cur_max = (vec.len() - 1, *vec.last().unwrap());
        }

        if *vec.last().unwrap() < cur_min.1 {
            cur_min = (vec.len() - 1, *vec.last().unwrap());
        }
    }

    return Some((cur_min, cur_max));
} 