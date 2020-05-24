#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

struct UnionFind(Vec<usize>);

#[allow(dead_code)]
impl UnionFind {
    fn new(len: usize) -> Self {
        Self((0..len).collect())
    }

    fn find(&mut self, i: usize) -> usize {
        if self.0[i] == i {
            i
        } else {
            let parent = self.0[i];
            self.0[i] = self.find(parent);
            self.0[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        self.0[root_i] = root_j;
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ps: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);

    for p in ps {
        if p.0 == 0 {
            uf.union(p.1, p.2);
        } else {
            if uf.find(p.1) == uf.find(p.2) {
                echo!("Yes");
            } else {
                echo!("No");
            }
        }
    }
}
