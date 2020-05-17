use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        qs: [(Usize1, Usize1, usize, usize); q],
    }

    let iter = (1..=m).combinations_with_replacement(n);
    let mut result = 0;

    for v in iter {
        let mut score = 0;
        for e in qs.iter() {
            if v[e.1] - v[e.0] == e.2 {
                score += e.3;
            }
        }
        if score > result {
            result = score;
        }
    }

    echo!(result);
}
