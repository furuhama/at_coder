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
    }

    let n = n % 10;

    if n == 3 {
        echo!("bon");
    } else if n == 0 || n == 1 || n == 6 || n == 8 {
        echo!("pon");
    } else {
        echo!("hon");
    }
}
