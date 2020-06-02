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
        a: usize,
        b: Chars,
    }

    let mut b100 = 0;
    for c in b.iter() {
        match c {
            '.' => continue,
            _ => {
                let d = c.to_digit(10).unwrap();
                b100 = b100 * 10 + d;
            }
        }
    }

    let result = a * b100 as usize / 100;

    echo!(result);
}
