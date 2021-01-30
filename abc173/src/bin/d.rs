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
        mut a: [usize; n],
    };

    a.sort();
    a.reverse();

    let mut ans = a[0];

    for i in 0..n - 2 {
        ans += a[i / 2 + 1];
    }

    echo!(ans);
}
