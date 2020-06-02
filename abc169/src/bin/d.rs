#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn prime_factors(n: usize) -> std::collections::HashMap<usize, usize> {
    let mut res = std::collections::HashMap::new();
    let mut n = n;

    for i in 2..(n as f32).sqrt() as usize + 1 {
        while n % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            n /= i;
        }
    }

    if n > 1 {
        res.insert(n, 1);
    }

    res
}

fn main() {
    input! {
        n: usize,
    }

    if n == 1 {
        echo!(0);
        return;
    }

    let pf = prime_factors(n);
    let mut result = 0;

    for &k in pf.values() {
        let mut cnt = k;
        for i in 1..=k {
            if cnt >= i {
                cnt -= i;
                result += 1;
            }
        }
    }

    echo!(result);
}
