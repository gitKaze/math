use crate::float::float::*;
use crate::int::int::*;

use std::{cmp::Ordering::*, fmt, iter::zip, sync::atomic::Ordering};
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed) + v1.exp);
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed) + v1.exp);
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed) + v1.exp);
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
        v1.denormalize(PRECISION.load(Ordering::Relaxed) + v1.exp);
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
                body: to_float(a[0]).body.pow(&(a[1].parse().unwrap())),
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
        if self.exp == 0 {
            return write!(f, "{}", self.body);
        }
        let (mut part1, part2) = if self.body.len() < self.exp as usize {
            (
                self.body.clone(),
                BigInt {
                    body: vec![0],
                    neg: self.body.neg,
                }
                .to_string(),
            )
        } else {
            (
                BigInt::from(&self.body.body[..(self.exp) as usize]),
                BigInt {
                    body: BigInt::from(&self.body.body[((self.exp) as usize)..]).body,
                    neg: self.body.neg,
                }
                .to_string(),
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
impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (mut v1, mut v2) = (self, other);
        if !v1.body.neg && v2.body.neg {
            return Some(Greater);
        } else if v1.body.neg && !v2.body.neg {
            return Some(Less);
        } else if v1.body.neg && v2.body.neg {
            (v1, v2) = (v2, v1);
        }
        let (l1, l2) = (
            v1.body.len() - v1.exp as usize,
            v2.body.len() - v2.exp as usize,
        );
        if l1 == l2 {
            for (a, b) in zip(v1.body.body.iter(), v2.body.body.iter()) {
                if a != b {
                    if a > b {
                        return Some(Greater);
                    } else {
                        return Some(Less);
                    }
                }
            }
            return Some(Equal);
        } else if l1 < l2 {
            return Some(Less);
        } else {
            return Some(Greater);
        };
    }
    fn ge(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => true,
            Some(Equal) => true,
            Some(Less) => false,
            _ => false,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => true,
            Some(Equal) => false,
            Some(Less) => false,
            _ => false,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => false,
            Some(Equal) => true,
            Some(Less) => true,
            _ => false,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => false,
            Some(Equal) => false,
            Some(Less) => true,
            _ => false,
        }
    }
}
impl PartialEq for BigFloat {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => false,
            Some(Equal) => true,
            Some(Less) => false,
            _ => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => true,
            Some(Equal) => false,
            Some(Less) => true,
            _ => false,
        }
    }
}
impl Ord for BigFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
impl Eq for BigFloat {}
impl PartialEq<&BigFloat> for BigFloat {
    fn eq(&self, other: &&Self) -> bool {
        return self.eq(*other);
    }
    fn ne(&self, other: &&Self) -> bool {
        return self.ne(*other);
    }
}
impl PartialOrd<&BigFloat> for BigFloat {
    fn partial_cmp(&self, other: &&Self) -> Option<std::cmp::Ordering> {
        return self.partial_cmp(*other);
    }
}
