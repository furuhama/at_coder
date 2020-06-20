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
        a: usize,
        b: usize,
    }

    if b <= a {
        echo!("delicious");
        return;
    }

    if b <= a + x {
        echo!("safe");
        return;
    }

    echo!("dangerous");
}
