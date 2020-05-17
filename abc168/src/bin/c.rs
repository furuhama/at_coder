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

    let hx = a * ((60.0 * h + m) / 360.0 * std::f64::consts::PI).cos();
    let hy = a * ((60.0 * h + m) / 360.0 * std::f64::consts::PI).sin();
    let mx = b * (m / 30.0 * std::f64::consts::PI).cos();
    let my = b * (m / 30.0 * std::f64::consts::PI).sin();
    let dx = hx - mx;
    let dy = hy - my;
    echo!((dx * dx + dy * dy).sqrt());
}
