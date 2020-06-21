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
        k: usize,
        mut p: [usize; n],
    }

    p.sort();
    let ans = p.into_iter().take(k).fold(0, |a, b| a + b);
    echo!(ans);
}
