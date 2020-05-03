#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        a: isize,
        b: isize,
        c: isize,
        ss: [String; n],
    }

    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut pattern = vec![];

    // MEMO: 先読みしたほうがいいかも
    fn phase(pattern: &mut Vec<String>, one: &mut isize, another: &mut isize, one_name: &str, another_name: &str) {
        if one >= another {
            *one -= 1;
            *another += 1;
            pattern.push(another_name.to_string());
        } else {
            *one += 1;
            *another -= 1;
            pattern.push(one_name.to_string());
        }
    }

    for i in 0..n {
        match &*ss[i] {
            "AB" => {
                phase(&mut pattern, &mut a, &mut b, "A", "B");
            },
            "BC" => {
                phase(&mut pattern, &mut b, &mut c, "B", "C");
            },
            _ => {
                phase(&mut pattern, &mut a, &mut c, "A", "C");
            },
        }

        if a * b * c < 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for e in &pattern {
        println!("{}", e);
    }
}
