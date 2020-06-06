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
        a: [usize; m],
    }

    let sum = a.into_iter().fold(0, |acc, e| acc + e);

    if sum > n {
        echo!(-1);
    } else {
        echo!(n - sum);
    }
}
