#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[derive(Clone, Copy)]
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

#[allow(dead_code)]
impl ModInt {
    // to change modulus, rewrite this value
    // (modulus should be a prime number)
    pub const MODULUS: u32 = 1_000_000_007;

    pub fn new(n: u32) -> Self {
        Self {
            value: n % Self::MODULUS,
        }
    }

    pub fn zero() -> ModInt {
        Self::new(0)
    }

    pub fn one() -> ModInt {
        Self::new(1)
    }

    pub fn pow(self, mut n: u32) -> ModInt {
        let mut t = ModInt::one();
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

// === modint ===

fn main() {
    input! {
        n: usize,
        abs: [(isize, isize); n],
    }

    let mut zeros = ModInt::zero();
    let mut vectors = std::collections::HashMap::new();

    for (a, b) in abs {
        if a == 0 && b == 0 {
            zeros += ModInt::one();
            continue;
        }
        let g = num::integer::gcd(a, b);
        let mut ag = a / g;
        let mut bg = b / g;

        // 第 1, 2 象限に集める
        if bg < 0 {
            ag *= -1;
            bg *= -1;
        }
        if bg == 0 && ag < 0 {
            ag *= -1;
        }

        if ag > 0 {
            let key = (ag, bg);
            vectors
                .entry(key)
                .or_insert((ModInt::zero(), ModInt::zero()));

            if let Some(x) = vectors.get_mut(&key) {
                x.0 += ModInt::one();
            }
        } else {
            let key = (bg, -ag);
            vectors
                .entry(key)
                .or_insert((ModInt::zero(), ModInt::zero()));

            if let Some(x) = vectors.get_mut(&key) {
                x.1 += ModInt::one();
            }
        }
    }

    let mut ans = ModInt::one();

    for (_, (l, r)) in vectors {
        let mut sec = ModInt::zero();

        if l.value() != 0 && r.value() != 0 {
            sec += ModInt::new(2).pow(l.value());
            sec += ModInt::new(2).pow(r.value());
            sec -= ModInt::one();
        } else if l.value() == 0 {
            sec += ModInt::new(2).pow(r.value());
        } else if r.value() == 0 {
            sec += ModInt::new(2).pow(l.value());
        }

        ans *= sec;
    }

    ans -= ModInt::one();
    ans += zeros;

    echo!(ans);
}
