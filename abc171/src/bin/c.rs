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
    }

    let n = n - 1; // 0-indexed
    let mut letter_count = 1;
    let mut pow = 26;
    let mut rest = n;
    while rest >= pow {
        rest -= pow;
        letter_count += 1;
        pow *= 26;
    }
    // letter_count 文字で rest 番目
    let mut rs = vec![];
    for _ in 0..letter_count {
        rs.push(rest % 26);
        if rest >= 26 {
            rest /= 26;
        } else {
            rest = 0;
        }
    }

    for i in rs.into_iter().rev() {
        let c = ('a' as u8 + i as u8) as char;

        print!("{}", c);
    }
    println!("");
}
