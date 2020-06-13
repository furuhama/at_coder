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
    }

    let mut ans = 0;

    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                let tmp = num::integer::gcd(a, b);
                ans += num::integer::gcd(tmp, c);
            }
        }
    }

    echo!(ans);
}
