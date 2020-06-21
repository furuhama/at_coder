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
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }

    let mut kv = std::collections::HashMap::new();
    let mut sum = 0;

    for e in a {
        *kv.entry(e).or_insert(0) += 1;
    }

    for (&k, &v) in kv.iter() {
        sum += k * v; // initialized sum
    }

    for (b, c) in bc {
        if let Some(count) = kv.remove(&b) {
            *kv.entry(c).or_insert(0) += count;

            if c >= b {
                sum += (c - b) * count;
            } else {
                sum -= (b - c) * count;
            }
        }
        echo!(sum);
    }
}
