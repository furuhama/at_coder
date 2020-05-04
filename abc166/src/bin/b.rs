#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut all = vec![false; n];

    for _ in 0..k {
        input! {
            d: usize,
            ts: [Usize1; d],
        }

        for t in ts {
            all[t] = true;
        }
    }

    let mut result = 0;
    for e in all {
        if e {
            continue;
        }

        result += 1;
    }

    echo!(result);
}
