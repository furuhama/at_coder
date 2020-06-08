#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

// (() のようなカッコで構成された文字列を受け取って
// (最終的な増減, 一番低いときいくつ下がるか) を返す
fn process(s: &str) -> (isize, isize) {
    let mut sum = 0;
    let mut lowest = 0;
    for c in s.chars() {
        match c {
            '(' => {
                sum += 1;
            }
            _ => {
                sum -= 1;
            }
        }

        lowest = std::cmp::min(lowest, sum);
    }

    (sum, lowest)
}

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let ss = s.iter().map(|s| process(s)).collect::<Vec<_>>();
    let mut positives = ss.iter().filter(|(sum, _)| *sum >= 0).collect::<Vec<_>>();
    let mut r_negatives = ss
        .iter()
        .filter(|(sum, _)| *sum < 0)
        .map(|(sum, pos)| (-sum, -sum + pos))
        .collect::<Vec<_>>();

    positives.sort_by_key(|e| -e.1); // 最下点までの下げ幅の小さい順
    r_negatives.sort_by_key(|e| -e.1); // 最下点までの下げ幅の小さい順

    let mut p_sum = 0;

    for p in positives {
        if p_sum + p.1 < 0 {
            echo!("No");
            return;
        }

        p_sum += p.0;
    }

    let mut n_sum = 0;

    for n in r_negatives {
        if n_sum + n.1 < 0 {
            echo!("No");
            return;
        }

        n_sum += n.0;
    }

    if p_sum != n_sum {
        echo!("No");
        return;
    }

    echo!("Yes");
}
