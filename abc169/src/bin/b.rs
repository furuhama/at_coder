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
        a: [usize;n],
    }

    let max = 10usize.pow(18);
    let result = a.iter().fold(1usize, |acc, e| acc.saturating_mul(*e));

    if result > max {
        echo!(-1);
    } else {
        echo!(result);
    }
}
