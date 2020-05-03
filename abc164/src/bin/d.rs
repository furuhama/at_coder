use proconio::{input, marker::Chars};
use num::BigUint;

fn main() {
    input! {
        s: Chars,
    }

    let len = s.len();

    if len < 4 {
        println!("0");
        return;
    }

    let mut mod_vec: Vec<Vec<usize>> = vec![Vec::new(); 2019];
    let mut rest: usize = 0;
    let modulus = BigUint::from(2019_usize);

    mod_vec[0].push(0);
    for i in 0..len {
        let digit = s[len - i - 1].to_digit(10).unwrap() as usize;
        let pow = BigUint::from(10_usize).modpow(&BigUint::from(i), &modulus);
        rest = (rest + digit * pow.to_u32_digits()[0] as usize) % 2019;
        mod_vec[rest].push(i);
    }

    let mut count = 0;

    for e in &mod_vec {
        if e.len() <= 1 {
            continue;
        }

        count += e.len() * (e.len() - 1) / 2;
    }

    println!("{}", count);
 }
