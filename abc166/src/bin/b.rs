use proconio::{input,fastout,marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ds: [(i32, Chars); k],
    }

    dbg!(n, k, ds);
}
