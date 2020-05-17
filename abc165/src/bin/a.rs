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
        k: usize,
        a: usize,
        b: usize,
    }

    for i in 1..=1000 {
        let k = k * i;
        if a <= k && k <= b {
            echo!("OK");
            return;
        }
    }
    echo!("NG");
}
