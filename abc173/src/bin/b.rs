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
        s: [String; n]
    };

    let mut ans = [0, 0, 0, 0]; // AC, WA, TLE, RE

    for e in s {
        match &*e {
            "AC" => ans[0] += 1,
            "WA" => ans[1] += 1,
            "TLE" => ans[2] += 1,
            _ => ans[3] += 1,
        };
    }

    print!("AC x {}\n", ans[0]);
    print!("WA x {}\n", ans[1]);
    print!("TLE x {}\n", ans[2]);
    print!("RE x {}", ans[3]);
}
