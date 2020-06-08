#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        c: isize,
        ss: [String; n],
    }

    let mut pattern = vec![];

    let mut abc = vec![a, b, c];
    let sum = a + b + c;

    if sum == 0 {
        println!("No");
        return;
    }

    fn run(large: usize, small: usize, abc: &mut Vec<isize>, pattern: &mut Vec<String>) {
        abc[large] -= 1;
        abc[small] += 1;
        let st = match small {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        pattern.push(st.to_string());
    }

    fn phase(
        pattern: &mut Vec<String>,
        abc: &mut Vec<isize>,
        ope: &str,
        next_ope: Option<&String>,
    ) {
        let (one, another, _theother) = match ope {
            "AB" => (0, 1, 2),
            "BC" => (1, 2, 0),
            _ => (2, 0, 1), // "AC"
        };

        if abc[one] == 0 || abc[another] == 0 {
            if abc[one] > abc[another] {
                run(one, another, abc, pattern);
            } else {
                run(another, one, abc, pattern);
            }
            return;
        }

        if let Some(n) = next_ope {
            // 先読み
            if n == "AB" {
                if one == 1 {
                    // 今が BC で次が AB なら B に 1 足す
                    run(another, one, abc, pattern);
                } else {
                    // 今が AC, AB で次が AB なら A に 1 足す
                    run(one, another, abc, pattern);
                }
            } else if n == "BC" {
                if one == 2 {
                    run(another, one, abc, pattern);
                } else {
                    run(one, another, abc, pattern);
                }
            } else {
                // "AC"
                if one == 0 {
                    run(another, one, abc, pattern);
                } else {
                    run(one, another, abc, pattern);
                }
            }

            return;
        }

        // どちらでも同じ
        run(one, another, abc, pattern);
    }

    for i in 0..n {
        phase(&mut pattern, &mut abc, &ss[i], ss.get(i + 1));

        if abc[0] < 0 || abc[1] < 0 || abc[2] < 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for e in &pattern {
        println!("{}", e);
    }
}
