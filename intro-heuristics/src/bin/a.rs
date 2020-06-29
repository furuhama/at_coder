#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use rand::prelude::*;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[allow(unused)]
fn calc(i: usize, day: usize, s: &Vec<Vec<isize>>, c: &Vec<isize>, last: &mut Vec<usize>) -> isize {
    last[i] = day;
    let up = s[day - 1][i];
    let mut down = 0;

    for j in 0..26 {
        down += (day - last[j]) as isize * c[j];
    }

    up - down
}

fn update(i: usize, j: usize, day: usize, s: &Vec<Vec<isize>>, c: &Vec<isize>) -> isize {
    let mut ans = s[day - 1][j] - s[day - 1][i];
    let c_diff = c[j] - c[i];
    let i = i as isize;
    let j = j as isize;
    ans += c_diff * (j * j - 25 * j - i * i + 25 * i);
    ans
}

fn main() {
    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
    }

    // let mut score = 0;
    // let mut last = vec![0; 26];

    let mut tmp_dec_max = (0, 0);
    for e in c.iter().enumerate() {
        if tmp_dec_max.1 < *e.1 {
            tmp_dec_max = (e.0, *e.1);
        }
    }

    let mut contests = vec![];

    for day in 1..=d {
        let mut tmp_inc_max = (0, 0);
        for i in 0..26 {
            if tmp_inc_max.1 < s[day - 1][i] {
                tmp_inc_max = (i, s[day - 1][i]);
            }
        }

        let target: usize;

        if tmp_inc_max.0 == tmp_dec_max.0 {
            target = tmp_inc_max.0;
        } else if tmp_inc_max.1 > tmp_dec_max.1 {
            target = tmp_inc_max.0;
        } else {
            target = tmp_dec_max.0;
        };
        contests.push(target);
        // score += calc(tmp_inc_max.0, day, &s, &c, &mut last);
    }

    let mut rng = rand::thread_rng();
    let ch_contest: Vec<_> = (0..26).collect();
    let ch_day: Vec<_> = (1..=d).collect();

    for _ in 0..1_000_000 {
        if let Some(contest) = ch_contest.choose(&mut rng) {
            if let Some(day) = ch_day.choose(&mut rng) {
                let before = contests[*day - 1];
                if update(before, *contest, *day, &s, &c) > 0 {
                    contests[*day - 1] = *contest;
                }
            };
        };
    }

    for e in contests {
        echo!(e + 1);
    }

    // score += 10_isize.pow(6);
    // score = std::cmp::max(0, score);

    // dbg!(score);
}
