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
        m: usize,
        mut k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    loop {
        if i == n - 1 && j == m - 1 {
            break;
        } else if i == n - 2 {
            if k < b[j] {
                break;
            }

            k -= b[j];
            j += 1;
        } else if j == m - 2 {
            if k < a[i] {
                break;
            }

            k -= a[i];
            i += 1;
        } else if a[i] == b[j] {
            a[i]
        }
    }
}
