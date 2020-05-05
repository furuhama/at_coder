use proconio::{input, marker::Chars};

macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    input! {
        s: Chars,
    }

    const MOD: usize = 2019;
    let mut dp = vec![0usize; MOD];
    dp[0] = 1;
    let mut pow = 1;
    let mut now = 0;
    let mut ans = 0;

    for digit in s.iter().rev() {
        let digit = digit.to_digit(10).unwrap() as usize;
        now = (now + pow * digit) % MOD;
        ans += dp[now];
        dp[now] += 1;
        pow = 10 * pow % MOD;
    }

    echo!(ans);
}
