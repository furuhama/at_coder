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
        _n: usize,
        s: Chars,
    }

    let mut rs = std::collections::HashSet::new();
    let mut gs = std::collections::HashSet::new();
    let mut bs = std::collections::HashSet::new();

    for (idx, &c) in s.iter().enumerate() {
        if c == 'R' {
            rs.insert(idx);
        } else if c == 'G' {
            gs.insert(idx);
        } else {
            bs.insert(idx);
        }
    }

    let mut ans = 0;

    for &r in rs.iter() {
        for &g in gs.iter() {
            let larger = std::cmp::max(r, g);
            let smaller = std::cmp::min(r, g);
            let diff = larger - smaller;

            let mut size = bs.len();

            if smaller >= diff {
                let b = smaller - diff;
                if let Some(_) = bs.get(&b) {
                    size -= 1;
                }
            }

            let b = larger + diff;

            if let Some(_) = bs.get(&b) {
                size -= 1;
            }

            if (larger + smaller) % 2 == 0 {
                let b = (larger + smaller) / 2;
                if let Some(_) = bs.get(&b) {
                    size -= 1;
                }
            }

            ans += size;
        }
    }

    echo!(ans);
}
