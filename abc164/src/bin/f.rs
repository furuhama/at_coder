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
        n: usize,
        ss: [usize; n],
        ts: [usize; n],
        us: [usize; n],
        vs: [usize; n],
    }

    let mut ans = vec![vec![None; n]; n];

    for (i, &s) in ss.iter().enumerate() {
        let u = us[i];

        if s == 0 {
            for e in ans[i].iter_mut() {
                *e = Some(u);
            }
        }
    }

    for (j, &t) in ts.iter().enumerate() {
        let v = vs[j];

        if t == 0 {
            for es in ans.iter_mut() {
                match es[j] {
                    Some(u) => {
                        es[j] = Some(u | v);
                    }
                    None => {
                        es[j] = Some(v);
                    }
                }
            }
        }
    }

    for (i, &s) in ss.iter().enumerate() {
        let u = us[i];

        if s == 1 {
            for (j, e) in ans[i].iter_mut().enumerate() {
                match *e {
                    Some(v) => {
                        if (u | v) != u {
                            echo!(-1);
                            return;
                        }
                    }
                    None => {
                        let v = vs[j];
                        *e = Some(u & v);
                    }
                }
            }
        }
    }

    for (j, &t) in ts.iter().enumerate() {
        let v = vs[j];

        if t == 1 {
            for (i, es) in ans.iter_mut().enumerate() {
                match es[j] {
                    Some(u) => {
                        if (u | v) != v {
                            echo!(-1);
                            return;
                        }
                    }
                    None => {
                        let u = us[i];
                        es[j] = Some(u & v);
                    }
                }
            }
        }
    }

    for es in ans.iter() {
        for (i, e) in es.iter().enumerate() {
            print!("{}", e.unwrap());
            if i != es.len() - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}
