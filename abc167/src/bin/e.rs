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
struct ModInt {
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

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp_p = vec![ModInt::zero(); n];
    let mut dp_c = vec![ModInt::zero(); k + 1];

    for i in 0..n {
        if i == 0 {
            dp_p[i] = ModInt::new(m as u32);
        } else {
            dp_p[i] = dp_p[i - 1] * ModInt::new(m as u32 - 1);
        }
    }

    for i in 0..=k {
        if i == 0 {
            dp_c[i] = ModInt::one();
        } else {
            dp_c[i] = dp_c[i - 1] * ModInt::new(n as u32 - i as u32) * ModInt::new(i as u32).inv();
        }
    }

    let mut ans = ModInt::zero();

    for i in 0..=k {
        ans += dp_c[i] * dp_p[n - i - 1];
    }

    echo!(ans.value());
}
