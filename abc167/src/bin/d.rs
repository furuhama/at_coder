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
    // input! {
    //     n: usize,
    //     k: usize,
    //     a: [Usize1; n],
    // }
    let n = 4;
    let k = 5;
    let a = vec![3, 2, 4, 1];

    let a = a.iter().map(|&e| e - 1).collect::<Vec<usize>>();

    let mut routes = vec![];
    let mut gone = std::collections::HashMap::new();

    let mut next = a[0];

    for _ in 0..n {
        match gone.get(&next) {
            Some(_) => {
                break;
            }
            None => {
                routes.push(next.clone());
                gone.insert(next.clone(), routes.len() - 1);
                next = a[next];
            }
        }
    }

    if k <= routes.len() {
        echo!(routes[k - 1] + 1);
    } else {
        let pre = next;
        let rest = k - pre;
        let loop_len = routes.len() - next;
        let rest = rest - (rest - 1) / loop_len * loop_len;
        let k = pre + rest;
        echo!(routes[k - 1] + 1);
    }
}
