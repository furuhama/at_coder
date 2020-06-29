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

fn calc_all(contests: &Vec<usize>, s: &Vec<Vec<isize>>, c: &Vec<isize>) -> isize {
    let mut last = vec![0; 26];
    let mut score = 0;

    for (day, &i) in contests.iter().enumerate() {
        score += calc(i, day + 1, s, c, &mut last);
    }

    score
}

fn main() {
    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
    }

    let now = std::time::SystemTime::now();

    let mut last = vec![0; 26];
    let mut score = 0;
    let mut contests = vec![];

    // greedy
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
        score += calc(best_i, day, &s, &c, &mut last);
        last[best_i] = day;
        contests.push(best_i);
    }

    let seed = [13; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

    while std::time::SystemTime::now().duration_since(now).unwrap()
        < std::time::Duration::new(1, 900)
    {
        if rng.gen_bool(0.5) {
            let day1 = rng.gen_range(1, d);
            let day2 = rng.gen_range(day1, std::cmp::min(day1 + 16, d));
            contests.swap(day1 - 1, day2 - 1);
            let new_score = calc_all(&contests, &s, &c);

            if new_score > score {
                score = new_score;
            } else {
                contests.swap(day1 - 1, day2 - 1);
            }
        } else {
            let contest = rng.gen_range(0, 26);
            let day = rng.gen_range(1, d + 1);
            let tmp = contests[day - 1];

            contests[day - 1] = contest;
            let new_score = calc_all(&contests, &s, &c);

            if new_score > score {
                score = new_score;
            } else {
                contests[day - 1] = tmp;
            }
        }
    }

    for e in contests {
        echo!(e + 1);
    }
    // dbg!(score);
}
