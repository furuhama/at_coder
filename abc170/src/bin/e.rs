#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[derive(Clone, Debug)]
pub struct MultiSet<T> {
    data: std::collections::BTreeMap<T, usize>,
}

impl<T: std::cmp::Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            data: std::collections::BTreeMap::new(),
        }
    }

    pub fn push(&mut self, v: T) {
        *self.data.entry(v).or_insert(0) += 1;
    }

    pub fn remove(&mut self, v: T) -> Result<(), ()> {
        match self.data.get_mut(&v) {
            Some(i) => {
                if *i == 1 {
                    self.data.remove(&v).unwrap();
                } else {
                    *i -= 1;
                }
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn max(&self) -> Option<&T> {
        self.data.iter().rev().next().map(|(k, _)| k)
    }

    pub fn min(&self) -> Option<&T> {
        self.data.iter().next().map(|(k, _)| k)
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
        };
    }

    for q in cd {
        let now = infants[q.0 - 1].1 - 1;
        let rate = infants[q.0 - 1].0;
        let next = q.1 - 1;

        infants[q.0 - 1] = (rate, next + 1);

        let now_max_before = max_sets[now].max().unwrap().clone();
        min_set.remove(now_max_before).unwrap();
        match max_sets[next].max() {
            Some(m) => min_set.remove(*m).unwrap(),
            _ => {}
        };

        max_sets[now].remove(rate).unwrap();
        max_sets[next].push(rate);

        match max_sets[now].max() {
            Some(m) => min_set.push(*m),
            _ => {}
        };
        let next_max_after = max_sets[next].max().unwrap().clone();
        min_set.push(next_max_after);

        echo!(min_set.min().unwrap());
    }
}
