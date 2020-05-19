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
    for e in ms {
        gs[e.0].push(e.1);
        gs[e.1].push(e.0);
    }

    let mut rs = vec![std::usize::MAX; n];
    rs[0] = 0;

    let mut vd = std::collections::VecDeque::new();
    vd.push_back(0);

    while let Some(to) = vd.pop_front() {
        for &g in &gs[to] {
            if rs[g] != std::usize::MAX {
                continue;
            }

            rs[g] = to;
            vd.push_back(g);
        }
    }

    for i in 0..n {
        if i == 0 {
            echo!("Yes");
        } else {
            echo!(rs[i] + 1);
        }
    }
}
