use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

// Wrapper around a value with it's occurence number for checking stability
#[derive(Clone, Copy)]
pub struct SortElem<T: Ord + Copy + Debug> {
    pub val: T,

    // occ should start at 0
    pub occ: usize,
}

impl <T: Ord + Copy + Debug + Hash> Hash for SortElem<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl <T: Ord + Copy + Debug> PartialEq for SortElem<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.val == other.val;
    }
}

impl <T: Ord + Copy + Debug> Eq for SortElem<T> {}

impl<T: Ord + Copy + Debug> PartialOrd for SortElem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T: Ord + Copy + Debug> Ord for SortElem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.val.cmp(&other.val);
    }
}

impl<T: Ord + Copy + Debug> Debug for SortElem<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({:?}, {:?})", self.val, self.occ);
    }
}

impl<T: Ord + Copy + Debug> TryInto<isize> for SortElem<T> where
    isize: TryFrom<T>
{
    type Error = String;

    fn try_into(self) -> std::result::Result<isize, Self::Error> {
        match isize::try_from(self.val) {
            Ok(n) => Ok(n),
            Err(_) => Err(String::from("Conversion Failed")),
        }
    }
}

pub type SortElemVec<T> = Vec<SortElem<T>>;

// Wraps every element in a Vec around a SortElem
pub fn to_sort_elem_vec<T: Ord + Hash + Eq + Copy + Debug>(vec: Vec<T>) -> SortElemVec<T> {
    let mut occ_map: HashMap<T, usize> = HashMap::new();
    let mut res: SortElemVec<T> = SortElemVec::new();

    for n in vec {
        if !occ_map.contains_key(&n) {
            occ_map.insert(n, 0);
        } else {
            occ_map.entry(n).and_modify(|x| *x += 1);
        }

        res.push(SortElem{
            val: n,
            occ: *occ_map.get(&n).unwrap(),
        });
    }
    
    return res;
}
