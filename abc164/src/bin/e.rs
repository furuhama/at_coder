#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::cmp::{min, Reverse};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        cs: [(Usize1, Usize1, usize, u64); m],
        exchanges: [(usize, u64); n],
    }

    let mut cities = vec![vec![]; n];
    for (u, v, a, b) in cs {
        cities[u].push((v, a, b));
        cities[v].push((u, a, b));
    }
    // n * a
    let max_money = 50 * 50;
    // dp[city][money] = time
    let mut dp = vec![vec![std::u64::MAX; max_money + 1]; n];
    // queue for (time, city, money) tuple
    let mut q = std::collections::BinaryHeap::new();

    dp[0][min(s, max_money)] = 0;
    q.push(Reverse((0, 0, min(s, max_money))));

    while let Some(Reverse((time, city, money))) = q.pop() {
        if dp[city][money] < time {
            continue;
        }

        // step forward to neighbor cities
        for &(another, cost_m, cost_t) in cities[city].iter() {
            // need more money!!
            if money < cost_m {
                continue;
            }

            let time = time + cost_t;
            let money = money - cost_m;

            if dp[another][money] > time {
                dp[another][money] = time;
                q.push(Reverse((time, another, money)));
            }
        }

        // step forward to money exchange
        let money = min(money + exchanges[city].0, max_money);
        let time = time + exchanges[city].1;
        if dp[city][money] > time {
            dp[city][money] = time;
            q.push(Reverse((time, city, money)));
        }
    }

    for i in dp.into_iter().skip(1) {
        let ans = i.into_iter().min().unwrap();
        echo!(ans);
    }
}
