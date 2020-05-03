use proconio::{input,fastout};
use num::pow::pow;

fn main() {
    input! {
        x: i64,
    }

    for i in -130..130 {
        let target = pow(i, 5) + x;

        for j in i+1..130 {
            if target == pow(j, 5) {
                println!("{} {}", j, i);
                return;
            }
        }
    }
}
