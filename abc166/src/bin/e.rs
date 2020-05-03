use proconio::{input,fastout};
use std::collections::HashMap;

fn main() {
    input! {
        n: isize,
        hs: [isize; n],
    }

    let mut hash = HashMap::<isize, Vec<isize>>::new();
    let mut results = 0;

    for i in 0..n {
        let sum = i + 1 + hs[i as usize];

        if let Some(n) = hash.get_mut(&sum) {
            n.push(i);
        } else {
            hash.insert(sum, vec![i]);
        };
    }

    for i in 0..n {
        let diff = i + 1 - hs[i as usize];

        if let Some(n) = hash.get_mut(&diff) {
            results += n.len();
        };
    }

    println!("{}", results);
}
