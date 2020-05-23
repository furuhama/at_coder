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
        a: isize,
        b: isize,
        _c: isize,
        k: isize,
    }

    if k <= a {
        echo!(k);
    } else if k <= a + b {
        echo!(a);
    } else {
        echo!(2 * a + b - k);
    }
}
