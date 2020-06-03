#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[derive(Clone, Copy, Debug)]
pub struct ModInt {
    value: u32,
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> Self::Output {
        let mut d = self.value + rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        ModInt::new(d)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: ModInt) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: ModInt) -> Self::Output {
        let mut d = self.value + Self::MODULUS - rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        ModInt::new(d)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: ModInt) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: ModInt) -> Self::Output {
        let d = self.value as u64 * rhs.value as u64 % Self::MODULUS as u64;

        ModInt::new(d as u32)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: ModInt) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for ModInt {
    type Output = ModInt;

    fn neg(self) -> Self::Output {
        let d = match self.value {
            0 => 0,
            _ => Self::MODULUS - self.value,
        };

        ModInt::new(d)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for ModInt {
    fn from(val: usize) -> Self {
        let d = (val % Self::MODULUS as usize) as u32;

        ModInt::new(d)
    }
}

impl PartialEq<Self> for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u32> for ModInt {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

#[allow(dead_code)]
impl ModInt {
    // to change modulus, rewrite this value
    // (modulus should be a prime number)
    pub const MODULUS: u32 = 998_244_353;

    pub fn new(n: u32) -> Self {
        Self {
            value: n % Self::MODULUS,
        }
    }

    pub fn pow(self, mut n: u32) -> ModInt {
        let mut t = ModInt::new(1);
        let mut s = self;
        while n > 0 {
            if n & 1 == 1 {
                t *= s;
            }
            s *= s;
            n >>= 1;
        }
        t
    }

    pub fn inv(self) -> ModInt {
        assert!(self.value > 0);
        self.pow(Self::MODULUS - 2)
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    input! {
        n: usize,
        s: usize,
        an: [usize; n],
    }

    let mut dp = vec![vec![ModInt::new(0); n]; s];

    for i in 0..s {
        if i + 1 == an[0] {
            dp[i][0] = ModInt::new(1);
        }
    }

    for i in 0..n + s {
        for j in 1..=i {
            let x = j;
            let y = i - x;

            if x > n - 1 || y > s - 1 {
                continue;
            }

            dp[y][x] = ModInt::new(2) * dp[y][x - 1];

            if y < an[x] - 1 {
                continue;
            } else if y == an[x] - 1 {
                dp[y][x] += ModInt::new(2).pow(x as u32);
            } else {
                let t = dp[y - an[x]][x - 1];
                dp[y][x] += t;
            }
        }
    }

    echo!(dp[s - 1][n - 1]);
}
