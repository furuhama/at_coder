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

    let n: u32 = r.reads::<u32>();
    let yen: u32 = r.readl::<u32>();

    'outer: for x in 0..n+1 {
        for y in 0..n+1-x {
            if 10000 * x + 5000 * y + 1000 * (n - x - y) == yen {
                println!("{} {} {}", x, y, n - x - y);
                break 'outer;
            }
            if x == n {
                println!("-1 -1 -1");
            }
        }
    }
}
