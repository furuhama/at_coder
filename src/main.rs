mod utils {
    use std::io::BufRead;
    use std::str::{self, FromStr};

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(reader: R) -> StdinReader<R> {
            StdinReader {
                reader: reader,
                buf: Vec::new(),
            }
        }

        // 区切り文字：スペース
        #[allow(dead_code)]
        pub fn reads<T: FromStr>(&mut self) -> T {
            self.read_until(b' ')
        }

        // 区切り文字：改行
        #[allow(dead_code)]
        pub fn readl<T: FromStr>(&mut self) -> T {
            self.read_until(b'\n')
        }

        pub fn read_until<T: FromStr>(&mut self, delim: u8) -> T {
            // self.bufに次のトークンをセットする
            loop {
                self.buf.clear();
                let len = self.reader
                    .read_until(delim, &mut self.buf)
                    .expect("failed reading bytes");
                match len {
                    0 => panic!("early eof"),
                    1 if self.buf[0] == delim => (), // 区切り文字だけなのでもう一度ループ
                    _ => {
                        // トークンが得られた
                        // 最後の文字が区切り文字なら削除
                        if self.buf[len - 1] == delim {
                            self.buf.truncate(len - 1);
                        }
                        break; // ループから脱出
                    }
                }
            }

            // 文字列をT型へパースする
            let elem = str::from_utf8(&self.buf).expect("invalid utf-8 string");
            elem.parse().unwrap_or_else(|_| panic!(format!("failed parsing: {}", elem)))
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut r = utils::StdinReader::new(stdin.lock());

    let n: u32 = r.readl::<u32>();
    let mut alice: u32 = 0;
    let mut bob: u32 = 0;
    let mut nums = Vec::with_capacity(n as usize);

    for i in 0..n {
        if i == n - 1 {
            nums.push(r.readl::<u32>());
        } else {
            nums.push(r.reads::<u32>());
        }
    }

    nums.sort();

    if n % 2 == 0 {
        for i in 0..n as usize {
            if i % 2 == 0 {
                bob += nums[i];
            } else {
                alice += nums[i];
            }
        }
    } else {
        for i in 0..n as usize {
            if i % 2 == 0 {
                alice += nums[i];
            } else {
                bob += nums[i];
            }
        }
    }

    println!("{}", alice - bob);
}
