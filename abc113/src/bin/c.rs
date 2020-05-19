macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        #[allow(unused_mut)]
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    input! {
        _n: usize,
        m: usize,
        city: [(usize, usize); m],
    }

    // 県 -> [(市, 年)]
    let mut cs = std::collections::HashMap::<usize, Vec<(usize, usize)>>::new();

    for (i, (p, y)) in city.into_iter().enumerate() {
        cs.entry(p).or_insert(Vec::<(usize, usize)>::new());

        if let Some(pairs) = cs.get_mut(&p) {
            pairs.push((i, y));
        };
    }

    let mut sorted = std::collections::HashMap::new();
    for (pref, cities) in &cs {
        let mut years: Vec<usize> = cities.iter().map(|&(_i, y)| y).collect();
        years.sort();
        sorted.insert(pref, years);
    }
    let mut ans = vec![String::new(); m];

    for (pref, cities) in &cs {
        let sorted_years = &sorted[pref];

        for &(i, y) in cities {
            let y_idx = sorted_years.binary_search(&y).unwrap();
            let s = format!("{:06}", &pref) + &format!("{:06}", y_idx + 1);
            ans[i] = s;
        }
    }

    for s in ans {
        echo!(s);
    }
}
