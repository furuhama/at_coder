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
        ab: [(usize, usize); n],
    }

    let mut mins = ab.iter().map(|e| e.0).collect::<Vec<_>>();
    mins.sort();
    let mut maxs = ab.iter().map(|e| e.1).collect::<Vec<_>>();
    maxs.sort();

    if n % 2 == 0 {
        let min_median = mins[n / 2 - 1] + mins[n / 2];
        let max_median = maxs[n / 2 - 1] + maxs[n / 2];
        echo!(max_median - min_median + 1);
    } else {
        let min_median = mins[(n + 1) / 2 - 1];
        let max_median = maxs[(n + 1) / 2 - 1];
        echo!(max_median - min_median + 1);
    }
}
