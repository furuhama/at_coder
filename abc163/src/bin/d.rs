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
pub struct Mint {
    value: u32,
}

impl std::ops::Add for Mint {
    type Output = Mint;
    fn add(self, rhs: Mint) -> Self::Output {
        let mut d = self.value + rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        Mint::new(d)
    }
}

impl std::ops::AddAssign for Mint {
    fn add_assign(&mut self, rhs: Mint) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Mint {
    type Output = Mint;
    fn sub(self, rhs: Mint) -> Self::Output {
        let mut d = self.value + Self::MODULUS - rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        Mint::new(d)
    }
}

impl std::ops::SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Mint) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Mint {
    type Output = Mint;
    fn mul(self, rhs: Mint) -> Self::Output {
        let d = self.value as u64 * rhs.value as u64 % Self::MODULUS as u64;

        Mint::new(d as u32)
    }
}

impl std::ops::MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Mint) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for Mint {
    type Output = Mint;

    fn neg(self) -> Self::Output {
        let d = match self.value {
            0 => 0,
            _ => Self::MODULUS - self.value,
        };

        Mint::new(d)
    }
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for Mint {
    fn from(val: usize) -> Self {
        let d = (val % Self::MODULUS as usize) as u32;

        Mint::new(d)
    }
}

impl PartialEq<Self> for Mint {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u32> for Mint {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

#[allow(dead_code)]
impl Mint {
    // to change modulus, rewrite this value
    // (modulus should be a prime number)
    pub const MODULUS: u32 = 1_000_000_007;

    pub fn new(n: u32) -> Self {
        Self {
            value: n % Self::MODULUS,
        }
    }

    pub fn pow(self, mut n: u32) -> Mint {
        let mut t = Mint::new(1);
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

    pub fn inv(self) -> Mint {
        assert!(self.value > 0);
        self.pow(Self::MODULUS - 2)
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn calc(i: Mint, n: Mint) -> Mint {
    n * i - i * i + i + Mint::new(1)
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let n_mod = Mint::new(n as u32);
    let mut sum = Mint::new(0);

    for i in k..=n+1 {
        sum += calc(Mint::new(i as u32), n_mod);
    }

    echo!(sum);
}
