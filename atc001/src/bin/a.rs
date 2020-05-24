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
        h: usize,
        w: usize,
        m: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];
    let mut start_h = 0;
    let mut start_w = 0;

    for (i, cs) in m.iter().enumerate() {
        for (j, &c) in cs.iter().enumerate() {
            if c == 's' {
                start_h = i;
                start_w = j;
            }
        }
    }

    let mut vd = std::collections::VecDeque::new();
    dp[start_h][start_w] = 1;
    vd.push_back((start_h, start_w));

    let mv = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((a, b)) = vd.pop_front() {
        match m[a][b] {
            'g' => {
                echo!("Yes");
                return;
            }
            '#' => {
                dp[a][b] = 1;
                continue;
            }
            _ => {
                dp[a][b] = 1;
                for (e_h, e_w) in mv.iter() {
                    let a = a as isize + e_h;
                    let b = b as isize + e_w;
                    if a < 0 || b < 0 || a as usize >= h || b as usize >= w {
                        continue;
                    }
                    if dp[a as usize][b as usize] != 0 {
                        continue;
                    }
                    vd.push_back((a as usize, b as usize));
                }
            }
        }
    }
    echo!("No");
}
