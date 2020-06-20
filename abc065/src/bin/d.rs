#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

pub struct UnionFind(Vec<usize>);

#[allow(dead_code)]
impl UnionFind {
    pub fn new(len: usize) -> Self {
        Self((0..len).collect())
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.0[i] == i {
            i
        } else {
            let parent = self.0[i];
            self.0[i] = self.find(parent);
            self.0[i]
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        self.0[root_i] = root_j;
    }
}

fn main() {
    input! {
        n: usize,
        cs: [(usize, usize); n],
    }

    let mut cs = cs
        .into_iter()
        .enumerate()
        .map(|(idx, (x, y))| (idx, x, y))
        .collect::<Vec<_>>();
    let mut ans = 0;
    let mut q = std::collections::BinaryHeap::new();

    cs.sort_by_key(|&(_, x, _)| x);

    for i in 1..n {
        let cost = cs[i].1 - cs[i - 1].1;
        q.push(std::cmp::Reverse((cost, cs[i - 1].0, cs[i].0)));
    }

    cs.sort_by_key(|&(_, _, y)| y);

    for i in 1..n {
        let cost = cs[i].2 - cs[i - 1].2;
        q.push(std::cmp::Reverse((cost, cs[i - 1].0, cs[i].0)));
    }

    let mut uf = UnionFind::new(n);

    while let Some(std::cmp::Reverse((cost, from, to))) = q.pop() {
        if uf.find(from) == uf.find(to) {
            continue;
        }

        uf.union(from, to);
        ans += cost;
    }

    echo!(ans);
}
