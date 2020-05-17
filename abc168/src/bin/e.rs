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
        abs: [(isize, isize); n],
    }

    let modular = 1000000007;

    let mut ts = vec![0; n];
    for i in 0..n {
        ts[i] = abs[i]
    }
}
