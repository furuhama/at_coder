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
        s: String,
        t: String,
    }

    let t = t[0..t.len() - 1].to_owned();

    if s == t {
        echo!("Yes");
    } else {
        echo!("No");
    }
}
