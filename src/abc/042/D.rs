macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const MOD: usize = 1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    }

    let ans = solver(h, w, a, b);

    println!("{}", ans);
}

fn solver(h: usize, w: usize, a: usize, b: usize) -> usize {
    let mut cached_before = 1;
    let mut cached_after = mod_patterns(h, w - b);

    let mut result = cached_before * cached_after;

    // start iteration only when h - a > 1
    for i in 2..(h - a + 1) {
        cached_before = cached_before * (i + b - 2) / (i - 1) % MOD;
        cached_after = cached_after * (h - i + 1) / (h - i + w - b) % MOD;

        result += cached_before * cached_after % MOD;
    }

    result % MOD
}

// pattern of going `h` x `w` tiles
fn mod_patterns(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        return 1;
    }

    let mut prod = 1;

    for i in 1..w {
        prod = prod * ((h + w - 2) - i + 1) / i % MOD;
    }

    prod
}
