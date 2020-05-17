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
        x: usize,
    }

    let mut base = 100;
    let mut years = 0;

    while base < x {
        base += base / 100;
        years += 1;
    }

    echo!(years);
}
