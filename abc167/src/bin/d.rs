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
    let mut loop_st = 0;

    for _ in 0..n {
        match gone.get(&next) {
            Some(n) => {
                loop_st = *n;
                break;
            }
            None => {
                routes.push(next);
                gone.insert(next, routes.len() - 1);
                next = a[next];
            }
        }
    }

    let loop_len = routes.len() - loop_st;

    let ans = if k < loop_st {
        routes[k]
    } else {
        let k = (k - loop_st) % loop_len;
        routes[(loop_st + k)]
    };

    echo!(ans + 1);
}
