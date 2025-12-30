use crate::float::float::*;
use crate::int::int::*;
use std::fmt;
use std::sync::atomic::Ordering;
#[allow(unused)]
use std::{cmp::*, ops::*};
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed));
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed));
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed));
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed));
        let result = BigFloat {
            body: &v1.body / &rhs.body,
            exp: v1.exp - PRECISION.load(Ordering::Relaxed),
        };
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
impl From<&String> for BigFloat {
    fn from(value: &String) -> BigFloat {
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

impl From<BigInt> for BigFloat {
    fn from(value: BigInt) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: value,
        };
        result.normalize();
        result
    }
}
impl From<&BigInt> for BigFloat {
    fn from(value: &BigInt) -> BigFloat {
        let mut result = BigFloat {
            exp: 0,
            body: value.clone(),
        };
        result.normalize();
        result
    }
}
#[allow(unused)]
fn parse_float<T: AsRef<str>>(input: T) -> BigFloat {
    let input: &str = input.as_ref();
    let input = input.trim();
    if !input.chars().all(|c| match c {
        '0'..='9' | '.' | '^' | 'e' | 'E' | '-' => true,
        _ => false,
    }) {
        panic!("wrong input")
    }
    let (e, eb, p, dot): (bool, bool, bool, bool) = (
        input.contains("e"),
        input.contains("E"),
        input.contains("^"),
        input.contains("."),
    );
    match (e, eb, p, dot) {
        (a, b, false, true) if a || b => to_float(input),
        (false, false, true, true) => {
            let a: Vec<&str> = input.splitn(2, "^").collect();
            if a[1].contains(".") {
                let power = to_float(a[1]);
                return pow_f(to_float(a[0]), power);
            }
            let result = BigFloat {
                body: pow(&to_float(a[0]).body, &(a[1].parse().unwrap())),
                exp: PRECISION.load(Ordering::Relaxed),
            };
            result
        }
        (false, false, false, true) => to_float(input),
        _ => BigFloat::from(BigInt::from(input)),
    }
}
#[allow(unused)]
fn to_float(_input: &str) -> BigFloat {
    BigFloat::default()
}
#[allow(unused)]
fn pow_f(base: BigFloat, power: BigFloat) -> BigFloat {
    todo!(
        "возвведение AGM  для а для этого надо сделать расчет pi желательно как константу которая считается когда изменяется точность "
    )
}
#[allow(unused)]
impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (mut part1, part2) = if self.body.len() < self.exp as usize {
            (self.body.clone(), BigInt::from(0).to_string())
        } else {
            (
                BigInt::from(&self.body.body[..(self.exp) as usize]),
                BigInt::from(&self.body.body[((self.exp) as usize)..]).to_string(),
            )
        };
        let chunk = BigInt::from("10^18");
        let mut string: Vec<String> = vec![part2];
        let mut temp: Vec<String> = Vec::new();

        for _ in 0..self.exp - 2 {
            part1 = part1 * &chunk;

            if part1.len() > self.exp as usize {
                let v = part1.body.pop().unwrap();
                trim(&mut part1.body);
                if part1 == BigInt::from(0) {
                    temp.push(v.to_string());
                    break;
                } else {
                    temp.push(format!("{:018}", v));
                }
            } else {
                temp.push(format!("{:018}", 0))
            }
        }
        let temp = temp.join("");
        string.push(temp);
        let result = string.join(".");
        write!(f, "{}", result)
    }
}
