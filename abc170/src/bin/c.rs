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
        x: usize,
        n: usize,
        p: [usize; n],
    }

    let mut hs = std::collections::HashSet::new();
    for e in p {
        hs.insert(e);
    }

    for i in 0..100 {
        let target = x - i;
        if let None = hs.get(&target) {
            echo!(target);
            return;
        }

        let target = x + i;
        if let None = hs.get(&target) {
            echo!(target);
            return;
        }
    }
}
