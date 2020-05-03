use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    let n = if a % d > 0 {
        a / d + 1
    } else {
        a / d
    };

    let m = if c % b > 0 {
        c / b + 1
    } else {
        c / b
    };

    if n >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
