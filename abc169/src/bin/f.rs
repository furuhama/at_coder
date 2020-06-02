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
        s: usize,
        an: [usize; n],
    }

    let mut dp = vec![vec![0; n]; s];
    dp[0][0] = if an[0] == 1 { 1 } else { 0 };
    let max = std::cmp::max(n, s);

    for i in 0..max {
        for j in 0..=i {
            let x = j;
            let y = i - x;
            // dp[y][x] =
        }
    }
}
