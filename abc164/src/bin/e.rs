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
        s: usize,
        e: [(Usize1, Usize1, usize, u64); m],
        p: [(usize, u64); n],
    }

    let mut graphs = vec![vec![]; n];
    for (a, b, c, d) in e {
        graphs[a].push((b, c, d));
        graphs[b].push((a, c, d));
    }

    // 都市数 N は最大 50
    // 運賃 A は最大 50
    // (N-1) * A だけあれば全都市回れる
    let max_cost = 50 * 49;

    // dp[頂点][お金] で dp する
    let mut dp = vec![vec![std::u64::MAX; max_cost + 1]; n];
    dp[0][std::cmp::min(max_cost, s)] = 0;

    // (時間, 都市, 金額) のタプルを入れる優先度付きキューを用意(小さい順にするため reverse している)
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0, 0, std::cmp::min(max_cost, s))));

    while let Some(std::cmp::Reverse((t, v, k))) = pq.pop() {
        if t > dp[v][k] {
            continue;
        }

        for &(u, a, b) in graphs[v].iter() {
            if a > k {
                continue;
            }

            let t = t + b;
            if t < dp[u][k - a] {
                dp[u][k - a] = t;
                pq.push(std::cmp::Reverse((t, u, k - a)));
            }
        }

        let t = t + p[v].1;
        let k = std::cmp::min(k + p[v].0, max_cost);
        if dp[v][k] > t {
            dp[v][k] = t;
            pq.push(std::cmp::Reverse((t, v, k)));
        }
    }

    for city in dp.into_iter().skip(1) {
        let ans = city.into_iter().min().unwrap();
        echo!(ans);
    }
}
