use core::cmp::{PartialEq, PartialOrd};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Rem, Sub},
};

impl BigInt {
    fn parse<T: AsRef<str>>(input: T) -> Self {
        let mut result = Self::default();
        let input: &str = input.as_ref();
        let input = input.trim();
        let (sing, input) = if input.starts_with("-") {
            (true, &input[1..])
        } else {
            (false, input)
        };
        if !input.chars().all(|c| match c {
            '0'..'9' | '.' | '^' | 'e' | 'E' => true,
            _ => false,
        }) {
            return BigInt::default();
        }
        let input = input.trim();
        result.neg = sing;
        let (e, eb, p, dot): (bool, bool, bool, bool) = (
            input.contains("e"),
            input.contains("E"),
            input.contains("^"),
            input.contains("."),
        );
        if p {
            let base_exp: Vec<&str> = input.splitn(2, "^").collect();
            let power: u64 = base_exp[1].parse().unwrap();
            result.body = pow(&str_bigint(&base_exp[0]), &power).body;
            //if BigInt::from(power) % Self::from(2) == Self::default() {
            //    result.neg = false
            //}
        } else if e || eb {
            let base_exp: Vec<&str> = if e {
                input.splitn(2, "e").collect()
            } else {
                input.splitn(2, "E").collect()
            };
            if dot {
                let baseparts: Vec<&str> = base_exp[0].split(".").collect();
                let power: u64 = &base_exp[1].parse().unwrap() - baseparts[1].len() as u64;
                if power == 0 {
                    result.body = (str_bigint(baseparts.concat())).body
                }
                let base = str_bigint(baseparts.concat());
                result.body = (base * pow10(&power)).body
            } else {
                result.body = (str_bigint(base_exp[0]) * pow10(&base_exp[1].parse().unwrap())).body
            }
        } else if dot {
            return Self::default();
        } else {
            result.body = str_bigint(input).body
        };
        return result;
    }
}
impl Sub<BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, _rhs: BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Sub<&BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Sub<BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, _rhs: Self) -> Self::Output {
        todo!("y")
    }
}
impl Mul<BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output {
        let result = self.mul1(&rhs);
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: result,
        };
        return result;
    }
}
impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &BigInt) -> Self::Output {
        let result = self.mul1(rhs);
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: result,
        };
        return result;
    }
}
impl Mul<&BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &BigInt) -> Self::Output {
        let result = self.mul1(&rhs);
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: result,
        };
        return result;
    }
}
impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.mul1(&rhs);
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: result,
        };
        return result;
    }
}
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        let sing = false;
        self.add_abs_ass(&rhs);
        self.neg = sing;
    }
}
impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        let sing = false;
        self.add_abs_ass(&rhs);
        self.neg = sing
    }
}
impl Add<&BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: &BigInt) -> Self::Output {
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: self.add_abs(rhs),
        };
        return result;
    }
}
impl Add<BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: self.add_abs(&rhs),
        };
        return result;
    }
}
impl Add<&BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: &Self) -> Self::Output {
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: self.add_abs(&rhs),
        };
        return result;
    }
}
impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: Self) -> Self::Output {
        let sing = false;
        let result = BigInt {
            neg: sing,
            body: self.add_abs(&rhs),
        };
        return result;
    }
}
impl Div<&BigInt> for &BigInt {
    type Output = BigInt;
    fn div(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Div<BigInt> for &BigInt {
    type Output = BigInt;
    fn div(self, _rhs: BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Div<&BigInt> for BigInt {
    type Output = BigInt;
    fn div(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Div<BigInt> for BigInt {
    type Output = BigInt;
    fn div(self, _rhs: Self) -> Self::Output {
        todo!("y")
    }
}
impl Rem<&BigInt> for &BigInt {
    type Output = BigInt;
    fn rem(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Rem<BigInt> for &BigInt {
    type Output = BigInt;
    fn rem(self, _rhs: BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Rem<&BigInt> for BigInt {
    type Output = BigInt;
    fn rem(self, _rhs: &BigInt) -> Self::Output {
        todo!("y")
    }
}
impl Rem<BigInt> for BigInt {
    type Output = BigInt;
    fn rem(self, _rhs: Self) -> Self::Output {
        todo!("y")
    }
}
impl PartialOrd for BigInt {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!("t")
    }
    fn ge(&self, _other: &Self) -> bool {
        todo!("u")
    }
    fn gt(&self, _other: &Self) -> bool {
        todo!("t")
    }
    fn le(&self, _other: &Self) -> bool {
        todo!("t")
    }
    fn lt(&self, _other: &Self) -> bool {
        todo!("t")
    }
}
impl PartialEq for BigInt {
    fn eq(&self, _other: &Self) -> bool {
        todo!("t")
    }
    fn ne(&self, _other: &Self) -> bool {
        todo!("t")
    }
}
impl fmt::Display for BigInt {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("y")
    }
}
impl Default for BigInt {
    fn default() -> Self {
        BigInt {
            neg: false,
            body: vec![0 as u64],
        }
    }
}
impl From<&str> for BigInt {
    fn from(input: &str) -> Self {
        Self::parse(input)
    }
}
impl From<&String> for BigInt {
    fn from(input: &String) -> Self {
        Self::parse(input)
    }
}
impl From<String> for BigInt {
    fn from(input: String) -> Self {
        Self::parse(input)
    }
}
impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        Self {
            neg: false,
            body: vec![value as u64],
        }
    }
}
impl From<isize> for BigInt {
    fn from(value: isize) -> Self {
        Self {
            neg: value < 0,
            body: vec![value.unsigned_abs() as u64],
        }
    }
}
impl From<i8> for BigInt {
    fn from(value: i8) -> Self {
        Self {
            neg: value < 0,
            body: vec![value.unsigned_abs() as u64],
        }
    }
}
impl From<i16> for BigInt {
    fn from(value: i16) -> Self {
        Self {
            neg: value < 0,
            body: vec![value.unsigned_abs() as u64],
        }
    }
}
impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        Self {
            neg: value < 0,
            body: vec![value.unsigned_abs() as u64],
        }
    }
}
impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        Self {
            neg: value < 0,
            body: vec![value.unsigned_abs() as u64],
        }
    }
}
impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        Self {
            neg: false,
            body: vec![value as u64],
        }
    }
}
impl From<u16> for BigInt {
    fn from(value: u16) -> Self {
        Self {
            neg: false,
            body: vec![value as u64],
        }
    }
}
impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        Self {
            neg: false,
            body: vec![value as u64],
        }
    }
}
impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        Self {
            neg: false,
            body: vec![value],
        }
    }
}
impl From<&u64> for BigInt {
    fn from(value: &u64) -> Self {
        Self {
            neg: false,
            body: vec![value.clone()],
        }
    }
}
impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        Self {
            neg: false,
            body: trim(&mut vec![value as u64, (value >> 64) as u64]),
        }
    }
}
