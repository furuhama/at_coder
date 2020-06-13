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
        c: Chars,
    }

    for e in c {
        if e == '7' {
            echo!("Yes");
            return;
        }
    }

    echo!("No");
}
