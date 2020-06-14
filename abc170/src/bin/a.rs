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
        x: [usize; 5],
    }

    for (i, n) in x.into_iter().enumerate() {
        if n == 0 {
            echo!(i + 1);
        }
    }
}
