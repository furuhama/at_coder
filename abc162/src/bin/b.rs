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

    let mut ans = 0;

    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        ans += i;
    }

    echo!(ans);
}
