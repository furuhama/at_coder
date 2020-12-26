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
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut sum = 0;
    let mut p = 0;

    while p < n && sum + a[p] <= k {
        sum += a[p];
        p += 1;
    }

    let mut ans = p;

    for (idx, e) in b.into_iter().enumerate() {
        sum += e;

        while p > 0 && sum > k {
            p -= 1;
            sum -= a[p];
        }

        if sum <= k {
            ans = std::cmp::max(ans, idx + 1 + p);
        }
    }

    echo!(ans);
}
