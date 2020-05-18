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

    match n % 10 {
        3 => {
            echo!("bon");
        }
        0 | 1 | 6 | 8 => {
            echo!("pon");
        }
        _ => {
            echo!("hon");
        }
    }
}
