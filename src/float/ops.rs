impl Add<BigFloat> for BigFloat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: self.body + rhs.body,
        };
        result
    }
}
impl Add<&BigFloat> for BigFloat {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: self.body + &rhs.body,
        };
        result
    }
}
impl Add<&BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: &BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body + &rhs.body,
        };
        result
    }
}
impl Add<BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body + &rhs.body,
        };
        result
    }
}
impl AddAssign<BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: Self) {
        self.body += rhs.body
    }
}
impl AddAssign<&BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: &Self) {
        self.body += &rhs.body
    }
}
impl Sub<BigFloat> for BigFloat {
    type Output = Self;
    fn sub(self, rhs: BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body - &rhs.body,
        };
        result
    }
}
impl Sub<&BigFloat> for BigFloat {
    type Output = Self;
    fn sub(self, rhs: &BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body - &rhs.body,
        };
        result
    }
}
impl Sub<BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body - &rhs.body,
        };
        result
    }
}
impl Sub<&BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: &BigFloat) -> Self::Output {
        let result = BigFloat {
            exp: self.exp,
            body: &self.body - &rhs.body,
        };
        result
    }
}
impl SubAssign<BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: Self) {
        self.body -= rhs.body
    }
}
impl SubAssign<&BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: &Self) {
        self.body -= &rhs.body
    }
}
impl Mul<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> Self::Output {
        let mut result = BigFloat {
            exp: self.exp + rhs.exp,
            body: &self.body * &rhs.body,
        };
        result.normalize();
        result
    }
}
impl Mul<&BigFloat> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: &BigFloat) -> Self::Output {
        let mut result = BigFloat {
            exp: self.exp + rhs.exp,
            body: &self.body * &rhs.body,
        };
        result.normalize();
        result
    }
}
impl Mul<&BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: &BigFloat) -> Self::Output {
        let mut result = BigFloat {
            exp: self.exp + rhs.exp,
            body: &self.body * &rhs.body,
        };
        result.normalize();
        result
    }
}
impl Mul<BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> Self::Output {
        let mut result = BigFloat {
            exp: self.exp + rhs.exp,
            body: &self.body * &rhs.body,
        };
        result.normalize();
        result
    }
}
impl Div<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigFloat) -> Self::Output {
        let mut v1 = self;
        v1.denormalize(PRECISION.load(Ordering::Relaxed) as i32);
        let mut result = BigFloat {
            body: &v1.body / &rhs.body,
            exp: v1.exp - rhs.exp,
        };
        result.normalize();
        result
    }
}
impl Div<&BigFloat> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &BigFloat) -> Self::Output {
        let mut v1 = self;
        v1.denormalize(PRECISION.load(Ordering::Relaxed) as i32);
        let mut result = BigFloat {
            body: &v1.body / &rhs.body,
            exp: v1.exp - rhs.exp,
        };
        result.normalize();
        result
    }
}
impl Div<BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigFloat) -> Self::Output {
        let mut v1 = self.clone();
        v1.denormalize(PRECISION.load(Ordering::Relaxed) as i32);
        let mut result = BigFloat {
            body: &v1.body / &rhs.body,
            exp: v1.exp - rhs.exp,
        };
        result.normalize();
        result
    }
}
impl Div<&BigFloat> for &BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &BigFloat) -> Self::Output {
        let mut v1 = self.clone();
        v1.denormalize(PRECISION.load(Ordering::Relaxed) as i32);
        let mut result = BigFloat {
            body: &v1.body / &rhs.body,
            exp: v1.exp - rhs.exp,
        };
        result.normalize();
        result
    }
}
impl Default for BigFloat {
    fn default() -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(0),
        };
        result.normalize();
        result
    }
}
impl From<u64> for BigFloat {
    fn from(value: u64) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<u128> for BigFloat {
    fn from(value: u128) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<u32> for BigFloat {
    fn from(value: u32) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<u16> for BigFloat {
    fn from(value: u16) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<i64> for BigFloat {
    fn from(value: i64) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<i32> for BigFloat {
    fn from(value: i32) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<i16> for BigFloat {
    fn from(value: i16) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<String> for BigFloat {
    fn from(value: String) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
impl From<&str> for BigFloat {
    fn from(value: &str) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: BigInt::from(value),
        };
        result.normalize();
        result
    }
}
