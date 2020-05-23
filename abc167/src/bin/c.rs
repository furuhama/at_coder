#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[allow(unused)]
fn dfs(
    idx: usize,
    cs: &Vec<Vec<usize>>,
    mut skills: Vec<usize>,
    c: usize,
    mut c_min: usize,
    n: usize,
    x: usize,
) -> isize {
    if idx >= n {
        if skills.iter().all(|&s| s >= x) {
            c_min = std::cmp::min(c, c_min);
            return c_min.clone() as isize;
        } else {
            return -1;
        }
    }

    let a = dfs(idx + 1, cs, skills.clone(), c, c_min, n, x);

    let c = c + cs[idx][0];
    for (i, s) in skills.iter_mut().enumerate() {
        *s += cs[idx][i + 1];
    }

    let b = dfs(idx + 1, cs, skills.clone(), c, c_min, n, x);

    if a != -1 && b != -1 {
        return std::cmp::min(a, b);
    } else if a == -1 && b == -1 {
        return -1;
    } else {
        // 片方だけ -1
        return std::cmp::max(a, b);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        cs: [[usize; m + 1]; n],
    }

    let ans = dfs(0, &cs, vec![0; m], 0, std::usize::MAX, n, x);

    echo!(ans);
}
