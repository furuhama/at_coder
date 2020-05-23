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
        k: usize,
        a: [Usize1; n],
    }

    let mut routes = vec![];
    let mut gone = std::collections::HashMap::new();

    let mut next = 0;
    let mut next_idx = 0;

    for _ in 0..n {
        match gone.get(&next) {
            Some(idx) => {
                next_idx = *idx;
                break;
            }
            None => {
                routes.push(next.clone());
                gone.insert(next.clone(), routes.len() - 1);
                next = a[next];
            }
        }
    }

    let k = k + 1_usize;

    if k <= routes.len() {
        echo!(routes[k - 1] + 1);
    } else {
        let pre = next;
        let rest = k - pre;
        let loop_len = routes.len() - next_idx;
        let rest = rest - (rest - 1) / loop_len * loop_len;
        let k = pre + rest;
        echo!(routes[k - 1] + 1);
    }
}
