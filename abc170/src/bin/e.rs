#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

pub trait MinMax<T> {
    fn mmax(&self) -> Option<&T>;
    fn mmin(&self) -> Option<&T>;
}

impl<T> MinMax<T> for std::collections::BTreeSet<T> {
    fn mmax(&self) -> Option<&T> {
        self.iter().rev().next()
    }

    fn mmin(&self) -> Option<&T> {
        self.iter().next()
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

    let mut max_sets = vec![std::collections::BTreeSet::new(); size];
    for i in infants.iter() {
        max_sets[i.1 - 1].insert(i.0);
    }

    let mut min_set = std::collections::BTreeSet::new();
    for max in max_sets.iter() {
        match max.mmax() {
            Some(m) => min_set.insert(*m),
            _ => true,
        };
    }

    for q in cd {
        let now = infants[q.0 - 1].1 - 1;
        let rate = infants[q.0 - 1].0;
        let next = q.1 - 1;

        infants[q.0 - 1] = (rate, next + 1);

        let now_max_before = max_sets[now].mmax().unwrap().clone();
        min_set.remove(&now_max_before);
        match max_sets[next].mmax() {
            Some(m) => min_set.remove(m),
            _ => true,
        };

        max_sets[now].remove(&rate);
        max_sets[next].insert(rate);

        match max_sets[now].mmax() {
            Some(m) => min_set.insert(*m),
            _ => true,
        };
        let next_max_after = max_sets[next].mmax().unwrap().clone();
        min_set.insert(next_max_after);

        echo!(min_set.mmin().unwrap());
    }
}
