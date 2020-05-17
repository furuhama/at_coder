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
        s: String,
    }

    if s.len() <= k {
        echo!(s);
    } else {
        let str = s[0..k].to_owned() + "...";
        echo!(str);
    }
}
