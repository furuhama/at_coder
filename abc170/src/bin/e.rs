#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

pub trait BinarySearchable<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

// allows binary_search for Slice
impl<T: Ord> BinarySearchable<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut lower = 0;
        let mut upper = self.len();

        while lower != upper {
            let mid = (lower + upper) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less => {
                    lower = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    upper = mid;
                }
            }
        }
        lower
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut lower = 0;
        let mut upper = self.len();

        while lower != upper {
            let mid = (lower + upper) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    lower = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    upper = mid;
                }
            }
        }
        lower
    }
}

#[derive(Clone)]
pub struct MultiSet<T> {
    data: Vec<T>,
}

impl<T: std::cmp::Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, i: T) {
        let idx = self.data.lower_bound(&i);
        self.data.insert(idx, i);
    }

    pub fn remove(&mut self, i: T) -> Result<(), ()> {
        match self.data.binary_search(&i) {
            Ok(idx) => {
                self.data.remove(idx);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn max(&self) -> Option<&T> {
        if self.data.len() == 0 {
            return None;
        }
        self.data.get(self.data.len() - 1)
    }

    pub fn min(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        infants: [(usize, usize); n],
        cd: [(usize, usize); q],
    }

    // 園児 i は infants[i-1].1 にいてレート infants[i-1].0
    let mut infants = infants;
    let size = 2 * 10_usize.pow(5);

    let mut max_sets = vec![MultiSet::new(); size];
    for i in infants.iter() {
        max_sets[i.1 - 1].push(i.0);
    }

    let mut min_set = MultiSet::new();
    for max in max_sets.iter() {
        match max.max() {
            Some(m) => min_set.push(*m),
            _ => {}
        }
    }

    for q in cd {
        let i = q.0 - 1;
        let now = infants[i].1 - 1;
        let rate = infants[i].0;
        let next = q.1 - 1;

        infants[i] = (rate, next + 1);

        let now_max_before = max_sets[now].max().unwrap().clone();
        min_set.remove(now_max_before).unwrap();
        match max_sets[next].max() {
            Some(m) => min_set.remove(*m).unwrap(),
            _ => {}
        }

        max_sets[now].remove(rate).unwrap();
        max_sets[next].push(rate);

        match max_sets[now].max() {
            Some(m) => min_set.push(*m),
            _ => {}
        }
        let next_max_after = max_sets[next].max().unwrap().clone();
        min_set.push(next_max_after);

        echo!(min_set.min().unwrap());
    }
}
