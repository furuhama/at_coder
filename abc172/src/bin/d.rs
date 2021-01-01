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
    let sum = |n: usize| -> usize { n * (n + 1) / 2 };

    for i in 1..=n {
        ans += sum(n / i) * i;
    }

    echo!(ans);
}
