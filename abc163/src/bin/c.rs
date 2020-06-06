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
        a: [usize; n-1],
    }

    let mut arr = vec![0; n];

    for e in a {
        arr[e-1] += 1;
    }

    for e in arr {
        echo!(e);
    }
}
