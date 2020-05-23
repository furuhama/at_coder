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
        x: usize,
        cs: [[usize; m + 1]; n],
    }

    let mut ans = std::usize::MAX;

    for i in 0..(1 << n) {
        let mut cost = 0;
        let mut skills = vec![0; m];

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                cost += cs[j][0];
                for k in 1..=m {
                    skills[k - 1] += cs[j][k];
                }
            };
        }

        if skills.iter().all(|&s| s >= x) {
            ans = std::cmp::min(ans, cost);
        }
    }

    if ans == std::usize::MAX {
        echo!(-1);
    } else {
        echo!(ans);
    }
}
