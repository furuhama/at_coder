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
        y: usize,
    }

    let y = y - 2 * x;
    if y < 0 {
        echo!("No");
        return;
    }

    if y % 2 != 0 {
        echo!("No");
        return;
    }

    if y > 2 * x {
        echo!("No");
        return;
    }

    echo!("Yes");
}
