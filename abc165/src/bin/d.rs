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
        a: usize,
        b: usize,
        n: usize,
    }

    let x = std::cmp::min(b - 1, n);

    let result = a * x / b - a * (x / b);

    echo!(result);
}
