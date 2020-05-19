use num::complex::Complex;
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
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let l = Complex::from_polar(&b, &(m / 60.0 * 2.0 * std::f64::consts::PI));
    let s = Complex::from_polar(&a, &((60.0 * h + m) / 720.0 * 2.0 * std::f64::consts::PI));
    let ans = (l - s).norm();

    echo!(ans);
}
