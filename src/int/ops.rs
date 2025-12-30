use crate::int::int::*;
use core::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::iter::zip;
#[allow(unused)]
use std::{fmt, ops::*};
impl BigInt {
    pub fn len(&self) -> usize {
        return self.body.len();
    }
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
            '0'..='9' | '.' | '^' | 'e' | 'E' => true,
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
            if (BigInt::from(power) % Self::from(2)) == Self::default() {
                result.neg = false
            }
        } else if e || eb {
            let base_exp: Vec<&str> = if e {
                input.splitn(2, "e").collect()
            } else {
                input.splitn(2, "E").collect()
            };
            if dot {
                let baseparts: Vec<&str> = base_exp[0].split(".").collect();
                let power: i64 = &base_exp[1].parse().unwrap() - baseparts[1].len() as i64 - 1;
                if power <= 0 {
                    result.body = (str_bigint(baseparts.concat())).body;
                    return result;
                }
                let base = str_bigint(baseparts.concat());
                result.body = (base * pow10(&(power as u64))).body
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
impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_sing_ass(&rhs)
    }
}
impl SubAssign<BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_sing_ass(&rhs)
    }
}
impl Sub<BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, rhs: BigInt) -> Self::Output {
        return self.sub_sing(&rhs);
    }
}
impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, rhs: &BigInt) -> Self::Output {
        return self.sub_sing(&rhs);
    }
}
impl Sub<&BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: &BigInt) -> Self::Output {
        return self.sub_sing(&rhs);
    }
}
impl Sub<BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: Self) -> Self::Output {
        return self.sub_sing(&rhs);
    }
}
impl Mul<BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output {
        let result = self.mul_man(&rhs);
        let sing = self.neg ^ rhs.neg;
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
        let result = self.mul_man(rhs);
        let sing = self.neg ^ rhs.neg;
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
        let result = self.mul_man(&rhs);
        let sing = self.neg ^ rhs.neg;
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
        let result = self.mul_man(&rhs);
        let sing = self.neg ^ rhs.neg;
        let result = BigInt {
            neg: sing,
            body: result,
        };
        return result;
    }
}
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        self.add_sing_ass(&rhs)
    }
}
impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        self.add_sing_ass(&rhs)
    }
}
impl Add<&BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: &BigInt) -> Self::Output {
        self.add_sing(rhs)
    }
}
impl Add<BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        self.add_sing(&rhs)
    }
}
impl Add<&BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: &Self) -> Self::Output {
        self.add_sing(rhs)
    }
}
impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: Self) -> Self::Output {
        self.add_sing(&rhs)
    }
}
impl Div<&BigInt> for &BigInt {
    type Output = BigInt;
    fn div(self, rhs: &BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (quo, _) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, true),
            body: quo,
        };
    }
}
impl Div<BigInt> for &BigInt {
    type Output = BigInt;
    fn div(self, rhs: BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (quo, _) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, true),
            body: quo,
        };
    }
}
impl Div<&BigInt> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: &BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (quo, _) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, true),
            body: quo,
        };
    }
}
impl Div<BigInt> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: Self) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (quo, _) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, true),
            body: quo,
        };
    }
}
impl Rem<&BigInt> for &BigInt {
    type Output = BigInt;
    fn rem(self, rhs: &BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (_, rem) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, false),
            body: rem,
        };
    }
}
impl Rem<BigInt> for &BigInt {
    type Output = BigInt;
    fn rem(self, rhs: BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (_, rem) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, false),
            body: rem,
        };
    }
}
impl Rem<&BigInt> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: &BigInt) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (_, rem) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, false),
            body: rem,
        };
    }
}
impl Rem<BigInt> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: Self) -> Self::Output {
        if BigInt::from(0) == self || BigInt::from(0) == rhs {
            return BigInt::default();
        }
        let (_, rem) = div_man(&self, &rhs);
        return BigInt {
            neg: div_sing(self.neg, rhs.neg, false),
            body: rem,
        };
    }
}
impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
impl Eq for BigInt {}
impl PartialEq<&BigInt> for BigInt {
    fn eq(&self, other: &&Self) -> bool {
        return self.eq(*other);
    }
    fn ne(&self, other: &&Self) -> bool {
        return self.ne(*other);
    }
}
impl PartialOrd<&BigInt> for BigInt {
    fn partial_cmp(&self, other: &&Self) -> Option<std::cmp::Ordering> {
        return self.partial_cmp(*other);
    }
}
impl PartialOrd<BigInt> for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (mut v1, mut v2) = (self, other);
        if !v1.neg && v2.neg {
            return Some(Ordering::Greater);
        } else if v1.neg && !v2.neg {
            return Some(Ordering::Less);
        } else if v1.neg && v2.neg {
            (v1, v2) = (v2, v1);
        }
        let (l1, l2) = (v1.body.len(), v2.body.len());
        if l1 == l2 {
            for (a, b) in zip(v1.body.iter(), v2.body.iter()) {
                if a != b {
                    if a > b {
                        return Some(Ordering::Greater);
                    } else {
                        return Some(Ordering::Less);
                    }
                }
            }
            return Some(Ordering::Equal);
        } else if l1 < l2 {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        };
    }
    fn ge(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => true,
            Some(Ordering::Less) => false,
            _ => false,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => false,
            Some(Ordering::Less) => false,
            _ => false,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => false,
            Some(Ordering::Equal) => true,
            Some(Ordering::Less) => true,
            _ => false,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => false,
            Some(Ordering::Equal) => false,
            Some(Ordering::Less) => true,
            _ => false,
        }
    }
}
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => false,
            Some(Ordering::Equal) => true,
            Some(Ordering::Less) => false,
            _ => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => false,
            Some(Ordering::Less) => true,
            _ => false,
        }
    }
}
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut number = self.clone();
        number.neg = false;
        if number == BigInt::from(0) {
            return write!(f, "0");
        }
        let mut rem: Vec<u64>;
        let mut string: Vec<String> = Vec::new();
        let chunk = BigInt::from("10^18");
        while number != BigInt::from(0) {
            (number.body, rem) = sh_div_abs(&number, &chunk);
            if number == BigInt::from(0) {
                string.push(rem[0].to_string());
                break;
            }
            string.push(format!("{:018}", rem[0]));
        }
        if self.neg == true {
            string.push("-".to_string());
        }
        string.reverse();
        let result = string.join("");
        write!(f, "{}", result)
    }
}
impl Default for BigInt {
    fn default() -> Self {
        BigInt {
            neg: false,
            body: vec![0u64],
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
impl From<&Vec<u64>> for BigInt {
    fn from(value: &Vec<u64>) -> Self {
        Self {
            neg: false,
            body: value.clone(),
        }
    }
}
impl From<&[u64]> for BigInt {
    fn from(value: &[u64]) -> Self {
        if value.is_empty() {
            return BigInt::default();
        }
        Self {
            neg: false,
            body: value.to_vec().clone(),
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
        let mut v = vec![value as u64, (value >> 64) as u64];
        trim(&mut v);
        Self {
            neg: false,
            body: v,
        }
    }
}
impl ShlAssign<u32> for BigInt {
    fn shl_assign(&mut self, other: u32) {
        shl_ass(self, &(other as u128));
    }
}
impl Shl<&u32> for BigInt {
    type Output = Self;
    fn shl(self, other: &u32) -> Self::Output {
        let result = shl(&self, &(*other as u128));
        result
    }
}
impl Shl<u32> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: u32) -> Self::Output {
        let result = shl(self, &(other as u128));
        result
    }
}
impl Shr<u32> for BigInt {
    type Output = Self;
    fn shr(self, other: u32) -> Self::Output {
        let result = shr(&self, &(other as u128));
        result
    }
}
impl Shr<u32> for &BigInt {
    type Output = BigInt;
    fn shr(self, other: u32) -> Self::Output {
        let result = shr(self, &(other as u128));
        result
    }
}
impl ShrAssign<u32> for BigInt {
    fn shr_assign(&mut self, other: u32) {
        shr_ass(self, &(other as u128));
    }
}
impl ShrAssign<&u32> for BigInt {
    fn shr_assign(&mut self, other: &u32) {
        shr_ass(self, &(*other as u128));
    }
}
impl ShlAssign<u128> for BigInt {
    fn shl_assign(&mut self, other: u128) {
        shl_ass(self, &(other as u128));
    }
}
impl Shl<&u128> for BigInt {
    type Output = Self;
    fn shl(self, other: &u128) -> Self::Output {
        let result = shl(&self, other);
        result
    }
}
impl Shl<u128> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: u128) -> Self::Output {
        let result = shl(self, &other);
        result
    }
}
impl Shr<u128> for BigInt {
    type Output = Self;
    fn shr(self, other: u128) -> Self::Output {
        let result = shr(&self, &other);
        result
    }
}
impl Shr<&u128> for &BigInt {
    type Output = BigInt;
    fn shr(self, other: &u128) -> Self::Output {
        let result = shr(self, &other);
        result
    }
}
impl ShrAssign<u128> for BigInt {
    fn shr_assign(&mut self, other: u128) {
        shr_ass(self, &other);
    }
}
impl ShrAssign<&u128> for BigInt {
    fn shr_assign(&mut self, other: &u128) {
        shr_ass(self, other);
    }
}
impl ShlAssign<u64> for BigInt {
    fn shl_assign(&mut self, other: u64) {
        shl_ass(self, &(other as u128));
    }
}
impl Shl<&u64> for BigInt {
    type Output = Self;
    fn shl(self, other: &u64) -> Self::Output {
        let result = shl(&self, &(*other as u128));
        result
    }
}
impl Shl<u64> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: u64) -> Self::Output {
        let result = shl(self, &(other as u128));
        result
    }
}
impl Shr<u64> for BigInt {
    type Output = Self;
    fn shr(self, other: u64) -> Self::Output {
        let result = shr(&self, &(other as u128));
        result
    }
}
impl Shr<u64> for &BigInt {
    type Output = BigInt;
    fn shr(self, other: u64) -> Self::Output {
        let result = shr(self, &(other as u128));
        result
    }
}
impl ShrAssign<u64> for BigInt {
    fn shr_assign(&mut self, other: u64) {
        shr_ass(self, &(other as u128));
    }
}
