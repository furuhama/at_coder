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
    }

    let mut gone = std::collections::HashSet::new();
    gone.insert(1);
    let mut next = a[0];
    let mut ans = 1;

    loop {
        if next == 2 {
            echo!(ans);
            return;
        }

        if let Some(_) = gone.get(&next) {
            echo!(-1);
            return;
        }

        gone.insert(next);
        next = a[next - 1];
        ans += 1;
    }
}
