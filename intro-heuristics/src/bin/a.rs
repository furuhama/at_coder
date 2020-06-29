#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
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

#[allow(unused)]
fn update(i: usize, j: usize, day: usize, s: &Vec<Vec<isize>>, c: &Vec<isize>) -> isize {
    let mut ans = s[day - 1][j] - s[day - 1][i];
    let c_diff = c[j] - c[i];
    let i = i as isize;
    let j = j as isize;
    ans += c_diff * (j * j - 25 * j - i * i + 25 * i + 27 * 13);
    ans
}

fn main() {
    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
    }

    let mut last = vec![0; 26];
    // let mut score = 0;
    let mut contests = vec![];

    for day in 1..=d {
        let mut max = std::isize::MIN;
        let mut best_i = 0;
        for i in 0..26 {
            let tmp = last[i];
            let score = calc(i, day, &s, &c, &mut last);
            if score > max {
                max = score;
                best_i = i;
            }
            last[i] = tmp;
        }
        // score += calc(best_i, day, &s, &c, &mut last);
        last[best_i] = day;
        contests.push(best_i);
    }

    // let mut rng = rand::thread_rng();
    // let ch_contest: Vec<_> = (0..26).collect();
    // let ch_day: Vec<_> = (1..=d).collect();

    // for _ in 0..1_000_000 {
    //     if let Some(contest) = ch_contest.choose(&mut rng) {
    //         if let Some(day) = ch_day.choose(&mut rng) {
    //             let before = contests[*day - 1];
    //             let update = update(before, *contest, *day, &s, &c);
    //             if update > 0 {
    //                 contests[*day - 1] = *contest;
    //                 // score += update;
    //                 // dbg!(score);
    //             }
    //         };
    //     };
    // }

    for e in contests {
        echo!(e + 1);
    }
    // dbg!(score);
}
