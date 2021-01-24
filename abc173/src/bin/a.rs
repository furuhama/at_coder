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
        n: usize
    };

    if n % 1_000 == 0 {
        echo!(0);
    } else {
        echo!(n / 1_000 * 1_000 + 1_000 - n);
    }
}
