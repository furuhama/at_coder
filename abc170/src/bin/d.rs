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
        a: [usize; n],
    }

    let size = 10_usize.pow(6);
    let mut dp = vec![0; size + 1];

    for i in a {
        if dp[i] > 1 {
            continue;
        }
        let mut tmp = i;
        let mut times = 1;
        while tmp <= size {
            dp[tmp] += times;
            times += 1;
            tmp += i;
        }
    }

    let mut ans = 0;

    for e in dp {
        if e == 1 {
            ans += 1;
        }
    }

    echo!(ans);
}
