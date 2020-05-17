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
        ms: [(Usize1, Usize1); m],
    }

    let mut gs = vec![vec![]; n];
    let mut results = vec![(0, 0); n];

    for e in ms {
        gs[e.0].push(e.1);
        gs[e.1].push(e.0);
    }

    let mut dp = vec![std::u64::MAX; n];
    let mut bh = std::collections::BinaryHeap::new();
    dp[0] = 0;
    bh.push(std::cmp::Reverse((0, 0)));

    while let Some(tup) = bh.pop() {
        let std::cmp::Reverse((cost, node)) = tup;
        if dp[node] < cost {
            continue;
        }

        let cost = cost + 1;
        for &g in &gs[node] {
            if dp[g] > cost {
                dp[g] = cost;
                bh.push(std::cmp::Reverse((cost, g)));
                results[g] = (node + 1, cost); // 出力する node 番号は 1-index
            }
        }
    }

    for (i, (node, cost)) in results.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if *node == 0 {
            echo!("No");
            return;
        }

        if results[node - 1].1 != cost - 1 {
            echo!("No");
            return;
        }
    }

    echo!("Yes");
    for (i, e) in results.iter().enumerate() {
        if i == 0 {
            continue;
        }
        echo!(e.0);
    }
}
