const MOD: usize = 998_244_353;

#[derive(Clone, Copy)]
struct ModInt(usize);

impl std::ops::Add for ModInt {
  type Output = ModInt;
  fn add(self, rhs: ModInt) -> Self::Output {
    let mut d = self.0 + rhs.0;
    if d >= MOD {
      d -= MOD;
    }
    ModInt(d)
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
    let mut d = self.0 + MOD - rhs.0;
    if d >= MOD {
      d -= MOD;
    }
    ModInt(d)
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
    ModInt((self.0 as u64 * rhs.0 as u64 % MOD as u64) as usize)
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
    ModInt(if self.0 == 0 { 0 } else { MOD - self.0 })
  }
}

impl std::fmt::Display for ModInt {
  fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::str::FromStr for ModInt {
  type Err = std::num::ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let val = s.parse::<usize>()?;
    Ok(ModInt::new(val))
  }
}

impl From<usize> for ModInt {
  fn from(val: usize) -> ModInt {
    ModInt((val % MOD as usize) as usize)
  }
}

#[allow(dead_code)]
impl ModInt {
  pub fn new(n: usize) -> ModInt {
    ModInt(n % MOD)
  }
  pub fn zero() -> ModInt {
    ModInt(0)
  }
  pub fn one() -> ModInt {
    ModInt(1)
  }
  pub fn pow(self, mut n: usize) -> ModInt {
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
    assert!(self.0 > 0);
    self.pow(MOD - 2)
  }
}
