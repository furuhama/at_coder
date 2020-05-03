use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        arr: [String; n],
    }

    let mut res = HashMap::new();
    for e in arr {
        res.insert(e, true);
    };
    println!("{}", res.len());
}
