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
        a: [usize; n],
    }

    let mut xor_all = a[0];
    for i in 1..n {
        xor_all ^= a[i];
    }

    for e in a {
        echo!(e ^ xor_all);
    }
}
