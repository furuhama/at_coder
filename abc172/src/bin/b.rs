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
        s: Chars,
        t: Chars,
    }

    let mut cnt = 0;

    for i in 0..s.len() {
        if s[i] != t[i] {
            cnt += 1;
        }
    }

    echo!(cnt);
}
