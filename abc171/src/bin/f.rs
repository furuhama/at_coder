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

pub struct Precalc {
    inv: Vec<Mint>,
    fact: Vec<Mint>,
    ifact: Vec<Mint>,
}

#[allow(dead_code)]
impl Precalc {
    pub fn new(n: usize) -> Self {
        let mut inv = vec![Mint::new(1); n + 1];
        let mut fact = vec![Mint::new(1); n + 1];
        let mut ifact = vec![Mint::new(1); n + 1];

        for i in 2..=n {
            fact[i] = fact[i - 1] * Mint::new(i as u32);
        }

        ifact[n] = fact[n].inv();

        if n > 0 {
            inv[n] = ifact[n] * fact[n - 1];
        }

        for i in (1..n).rev() {
            ifact[i] = ifact[i + 1] * Mint::new((i + 1) as u32);
            inv[i] = ifact[i] * fact[i - 1];
        }

        Self { inv, fact, ifact }
    }

    pub fn inv(&self, n: usize) -> Mint {
        assert!(n > 0);
        self.inv[n]
    }

    pub fn fact(&self, n: usize) -> Mint {
        self.fact[n]
    }

    pub fn ifact(&self, n: usize) -> Mint {
        self.ifact[n]
    }

    pub fn comb(&self, n: usize, k: usize) -> Mint {
        if k > n {
            return Mint::new(0);
        }
        self.fact[n] * self.ifact[k] * self.ifact[n - k]
    }
}

fn main() {
    input! {
        k: usize,
        s: Chars,
    }

    let mut extra = Mint::new(0);
    let n = s.len() + k;
    let pc = Precalc::new(n);
    for i in 0..s.len() {
        extra += pc.comb(n, i) * Mint::new(25).pow((n - i) as u32);
    }

    let ans = Mint::new(26).pow(n as u32) - extra;
    echo!(ans);
}
