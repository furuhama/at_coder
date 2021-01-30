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
        k: usize,
        b: [Chars; h],
    };

    let board: Vec<Vec<bool>> = b
        .into_iter()
        .map(|col| {
            col.into_iter()
                .map(|c| if c == '#' { true } else { false })
                .collect()
        })
        .collect();

    let mut ans = 0;
    for i in 0..(1 << h) {
        for j in 0..(1 << w) {
            let mut cnt = 0;
            for (y, row) in board.iter().enumerate() {
                for (x, &b) in row.into_iter().enumerate() {
                    // (i >> y) & 1) & ((j >> x) & 1 は塗らない場合に 1 になる
                    if ((i >> y) & 1) & ((j >> x) & 1) == 1 && b {
                        cnt += 1;
                    }
                }
            }
            if cnt == k {
                ans += 1;
            }
        }
    }

    echo!(ans);
}
