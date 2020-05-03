use proconio::{input,fastout};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        hs: [usize; n],
        ms: [(usize, usize); m],
    }

    let mut goods = 0;
    let mut pairs = vec![HashMap::new(); n];

    for i in 0..m {
        let left = ms[i].0;
        let right = ms[i].1;

        pairs[left-1].insert(right, ());
        pairs[right-1].insert(left, ());
    }

    'outer: for i in 0..n {
        for neighbor in pairs[i].keys() {
            if hs[i] <= hs[*neighbor-1] {
                continue 'outer;
            }
        }

        goods += 1;
    }

    println!("{}", goods);
}
