#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

// Returns prime numbers until n
pub fn primes(n: usize) -> Vec<usize> {
    let mut arr = vec![true; n + 1];
    let mut primes = vec![];
    arr[0] = false; // 0 is NOT a prime number
    arr[1] = false; // 1 is NOT a prime number
    for p in 2..=n {
        if !arr[p] {
            continue;
        }

        primes.push(p);

        let mut mul_p = 2 * p;
        while mul_p <= n {
            arr[mul_p] = false;
            mul_p += p;
        }
    }

    primes
}

// Returns prime factors of n
pub fn prime_factors(n: usize) -> std::collections::HashMap<usize, usize> {
    let mut n = n;
    let mut pf = std::collections::HashMap::new();
    let ps = primes((n as f64).sqrt() as usize + 2);

    for p in ps {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }

        if count > 0 {
            pf.insert(p, count);
        }
    }

    // for case n is a prime number
    if n > 1 {
        pf.insert(n, 1);
    }

    pf
}

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 1; // 1 の分

    for i in 2..=n {
        let pf = prime_factors(i);

        let mut factors = 1;
        for (_k, v) in pf {
            factors *= v + 1;
        }

        ans += factors * i;
    }

    echo!(ans);
}
