use core::cmp::{Ord, PartialEq, PartialOrd};
use std::cmp::Ordering;
use std::ops::SubAssign;
#[allow(unused)]
use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Rem, Shl, ShlAssign, Shr, ShrAssign, Sub},
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("y")
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
impl ShlAssign<u32> for BigInt {
    fn shl_assign(&mut self, other: u32) {
        if other > 63 {
            let app: Vec<u64> = vec![0; (other / 64) as usize];
            self.body.splice(0..0, app);
        }
        let shift = other % 64;
        let mut carry = 0u64;
        for a in self.body.iter_mut() {
            let temp = (*a as u128) << shift;
            *a = (temp as u64) | carry;
            carry = (temp >> 64) as u64;
        }
        if carry != 0 {
            self.body.push(carry)
        }
    }
}
impl Shl<&u32> for BigInt {
    type Output = Self;
    fn shl(self, other: &u32) -> Self::Output {
        let mut result = BigInt {
            neg: self.neg,
            body: self.body.clone(),
        };
        if other > &63 {
            let app: Vec<u64> = vec![0; (other / 64) as usize];
            result.body.splice(0..0, app);
        }
        let shift = other % 64;
        let mut carry = 0u64;
        for a in result.body.iter_mut() {
            let temp = (*a as u128) << shift;
            *a = (temp as u64) | carry;
            carry = (temp >> 64) as u64;
        }
        if carry != 0 {
            result.body.push(carry)
        }
        result
    }
}
impl Shl<u32> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: u32) -> Self::Output {
        let mut result = BigInt {
            neg: self.neg,
            body: self.body.clone(),
        };
        if other > 63 {
            let app: Vec<u64> = vec![0; (other / 64) as usize];
            result.body.splice(0..0, app);
        }
        let shift = other % 64;
        let mut carry = 0u64;
        for a in result.body.iter_mut() {
            let temp = (*a as u128) << shift;
            *a = (temp as u64) | carry;
            carry = (temp >> 64) as u64;
        }
        if carry != 0 {
            result.body.push(carry)
        }
        result
    }
}
impl Shr<u32> for BigInt {}
impl Shr<u32> for BigInt {}
impl ShrAssign<u32> for BigInt {}
impl ShrAssign<&u32> for BigInt {}
