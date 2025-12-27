#[allow(unused)]
use rayon::iter::*;
#[allow(unused)]
use rayon::prelude::*;
use std::iter::zip;

include!("ops.rs");
const POWS10: &[u64] = &[
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
];
#[derive(Debug, Clone)]
pub struct BigInt {
    pub neg: bool,
    pub body: Vec<u64>,
}
impl BigInt {
    fn add_sing(&self, other: &Self) -> BigInt {
        match (self.neg, other.neg) {
            (true, true) => BigInt {
                neg: true,
                body: self.add_abs(other),
            },
            (false, false) => BigInt {
                neg: false,
                body: self.add_abs(other),
            },
            (true, false) => {
                if cmp_abs(self, other) == 1 {
                    BigInt {
                        neg: true,
                        body: self.sub_abs(other),
                    }
                } else {
                    BigInt {
                        neg: false,
                        body: other.sub_abs(self),
                    }
                }
            }
            (false, true) => {
                if cmp_abs(self, other) == 1 {
                    BigInt {
                        neg: false,
                        body: self.sub_abs(other),
                    }
                } else {
                    BigInt {
                        neg: true,
                        body: other.sub_abs(self),
                    }
                }
            }
        }
    }
    fn add_sing_ass(&mut self, other: &Self) {
        match (self.neg, other.neg) {
            (true, true) => {
                self.add_abs_ass(other);
                self.neg = true;
            }
            (false, false) => {
                self.add_abs_ass(other);
                self.neg = false;
            }
            (true, false) => {
                if cmp_abs(&self, other) == 1 {
                    self.sub_abs_ass(other);
                    self.neg = true;
                } else {
                    self.sub_abs_ass(other);
                    self.neg = false;
                }
            }
            (false, true) => {
                if cmp_abs(&self, other) == 1 {
                    self.sub_abs_ass(other);
                    self.neg = false;
                } else {
                    self.sub_abs_ass(other);
                    self.neg = true;
                }
            }
        }
    }
    fn sub_sing_ass(&mut self, other: &Self) {
        if self.neg && other.neg {
            if *self < other {
                self.sub_abs_ass(other);
                self.neg = false
            } else {
                self.sub_abs_ass(other);
                self.neg = true
            }
        } else if !self.neg && other.neg {
            self.add_abs_ass(other);
            self.neg = false
        } else if self.neg && !other.neg {
            self.add_abs_ass(other);
            self.neg = false
        } else {
            if *self < other {
                self.sub_abs_ass(other);
                self.neg = true
            } else {
                self.sub_abs_ass(other);
                self.neg = false
            }
        };
    }
    fn sub_sing(&self, other: &Self) -> BigInt {
        if self.neg && other.neg {
            if self < other {
                return BigInt {
                    neg: false,
                    body: self.sub_abs(other),
                };
            } else {
                return BigInt {
                    neg: true,
                    body: self.sub_abs(other),
                };
            }
        } else if !self.neg && other.neg {
            return BigInt {
                neg: false,
                body: self.add_abs(other),
            };
        } else if self.neg && !other.neg {
            return BigInt {
                neg: false,
                body: self.add_abs(other),
            };
        } else {
            if self < other {
                return BigInt {
                    neg: true,
                    body: self.sub_abs(other),
                };
            } else {
                return BigInt {
                    neg: false,
                    body: self.sub_abs(other),
                };
            }
        };
    }
    #[inline(always)]
    fn sub_abs(&self, other: &Self) -> Vec<u64> {
        let (v1, v2) = if cmp_abs(self, other) == -1 {
            (self, other)
        } else if cmp_abs(self, other) == 0 {
            return vec![0];
        } else {
            (other, self)
        };
        let mut result = v1.body.clone();
        let mut borrow: bool = false;
        for (a, b) in zip(result.iter_mut(), v2.body.iter()) {
            (*a, borrow) = a.overflowing_sub(if borrow { *b + 1 } else { *b })
        }
        if borrow {
            for a in result[v2.body.len()..].iter_mut() {
                (*a, borrow) = a.overflowing_sub(1);
                if !borrow {
                    break;
                }
            }
        }
        trim(&mut result)
    }
    #[inline(always)]
    fn sub_abs_ass(&mut self, other: &Self) {
        if *self < other {
            let mut borrow: bool = false;
            self.body.resize(other.body.len(), 0);
            for (a, b) in zip(self.body.iter_mut(), other.body.iter()) {
                (*a, borrow) = b.borrowing_sub(*a, borrow);
                if a == &0 && !borrow {
                    break;
                }
            }
            self.body = trim(&mut self.body);
            return;
        };
        let mut borrow: bool = false;
        for (a, b) in zip(self.body.iter_mut(), other.body.iter()) {
            (*a, borrow) = a.overflowing_sub(if borrow { *b + 1 } else { *b })
        }
        if borrow {
            for a in self.body[other.body.len()..].iter_mut() {
                (*a, borrow) = a.overflowing_sub(1);
                if !borrow {
                    break;
                }
            }
        }
        self.body = trim(&mut self.body);
    }
    #[inline(always)]
    fn add_abs_ass(&mut self, other: &Self) {
        let (mut l1, mut l2) = (self.body.len(), other.body.len());
        if l1 < l2 {
            self.body.resize(l2, 0);
            (l1, l2) = (l2, l2)
        };
        let mut carry = false;
        for (a, &b) in zip(self.body.iter_mut(), other.body.iter()) {
            (*a, carry) = a.carrying_add(b, carry);
        }
        if carry && l1 > l2 {
            for i in self.body[l2..].iter_mut() {
                (*i, carry) = i.carrying_add(0, carry);
                if !carry {
                    break;
                }
            }
        }
        if carry {
            self.body.push(1);
        }
    }
    #[inline(always)]
    fn add_abs(&self, other: &Self) -> Vec<u64> {
        let (l1, l2) = (self.body.len(), other.body.len());
        let (v1, v2, l1, mut l2) = if l1 > l2 {
            (self, other, l1, l2)
        } else {
            (other, self, l2, l1)
        };
        let mut result = v1.body.clone();
        let mut carry = false;
        for (a, &b) in zip(result.iter_mut(), v2.body.iter()) {
            (*a, carry) = a.carrying_add(b, carry);
        }
        while carry {
            if l2 == l1 {
                result.push(1u64);
                break;
            }
            (result[l2], carry) = result[l2].carrying_add(0, carry);
            l2 += 1
        }
        return result;
    }
    fn mul_man(&self, v2: &Self) -> Vec<u64> {
        let (l1, l2) = (self.body.len(), v2.body.len());
        match (l1, l2) {
            (a, b) if a < 50 || b < 50 => self.mul1(v2),
            (a, b) if a < 1500 || b < 1500 => self.mul1(v2),
            (a, b) if a < 3000 && b < 3000 => self.mul1(v2),
            _ => self.mul1(v2),
        }
    }
    #[inline(always)]
    fn mul1(&self, other: &Self) -> Vec<u64> {
        let (s1, s2) = (self.body.len(), other.body.len());
        let mut result: Vec<u64> = vec![0; s1 + s2];
        let mut c1: bool;
        let mut c2 = false;
        for (i1, &v1) in zip(0..s1, self.body.iter()) {
            for (i2, &v2) in zip(0..s2, other.body.iter()) {
                if v1 == 0 || v2 == 0 {
                    if c2 {
                        result[i1 + i2] += 1;
                        c2 = false
                    }
                    continue;
                }
                let (low, hi) = v1.carrying_mul(v2, 0);
                (result[i1 + i2], c1) = result[i1 + i2].carrying_add(low, c2);
                (result[i1 + i2 + 1], c2) = result[i1 + i2 + 1].carrying_add(hi, c1);
            }
        }
        result = trim(&mut result);
        return result;
    }
}
fn cmp_abs(v1: &BigInt, v2: &BigInt) -> i8 {
    match v1.body.cmp(&v2.body) {
        Ordering::Greater => 1,
        Ordering::Equal => 0,
        Ordering::Less => -1,
    }
}
#[inline(always)]
fn trim(val: &mut Vec<u64>) -> Vec<u64> {
    let mut s = val.len() - 1;
    while s != 0 && val[s] == 0 {
        val.pop();
        s -= 1
    }
    return std::mem::take(val);
}
fn str_bigint<T: AsRef<str>>(str: T) -> BigInt {
    let str = str.as_ref();
    let (mut start, mut end, size, mut result) = (0, 0, str.len(), BigInt::default());
    while start != size {
        end = std::cmp::min(end + 18, size);
        let segment = &str[start..end];
        start = end;
        let mult = POWS10[segment.len()];
        let segment: u64 = segment.parse().unwrap();
        result = result * BigInt::from(mult);
        result += BigInt::from(segment);
    }
    return result;
}
pub fn pow10(power: &u64) -> BigInt {
    if power < &0 {
        return BigInt::default();
    }
    let mut res = BigInt::from(1);
    let mut b: BigInt = BigInt::from(10);
    let mut n = power.clone();
    while n != 0 {
        if (n & 1) == 1 {
            res = &res * &b;
            if n == 1 {
                break;
            }
            b = &b * &b;
        } else {
            b = &b * &b;
        }
        n >>= 1;
    }

    return res;
}
pub fn pow(base: &BigInt, power: &u64) -> BigInt {
    if power < &0 {
        return BigInt::default();
    }
    if BigInt::from(0) == base {
        return BigInt::default();
    } else if BigInt::from(1) == base || power == &0 {
        return BigInt::from(1);
    } else if power == &1 {
        return BigInt {
            neg: (false),
            body: (base.body.clone()),
        };
    }
    let mut res = BigInt {
        neg: (base.neg),
        body: (vec![1]),
    };
    if power % 2 == 0 {
        res.neg = false
    }
    let mut b: BigInt = BigInt::default();
    b.body.clone_from(&base.body);
    let mut n = power.clone();
    while n != 0 {
        if (n & 1) == 1 {
            res = &res * &b;
            if n == 1 {
                break;
            }
            b = &b * &b;
        } else {
            b = &b * &b;
        }
        n >>= 1;
    }

    return res;
}
fn div_ww_w(hi: &u64, low: &u64, divs: &u64) -> (u128, u128) {
    let dividend = ((*hi as u128) << 64) | (*low as u128);
    return (
        (dividend / *divs as u128), //quotient
        (dividend % *divs as u128), //remainder
    );
}

fn div_abs(v1: &BigInt, v2: &BigInt) -> (Vec<u64>, Vec<u64>) {
    let ld = v2.body.last().unwrap().leading_zeros();
    let mut dividend = v1 << ld;
    dividend.body.push(0);
    let divisor = v2 << ld;
    let (n, m) = (divisor.body.len(), dividend.body.len());
    let mut result: Vec<u64> = vec![0; m - n + 1];
    for i in (0..(m - n)).rev() {
        let (h1, h2, d1) = (
            dividend.body[i + n],
            dividend.body[i + n - 1],
            divisor.body[n - 1],
        );
        let (d2, h3) = (dividend.body[i + n - 2], divisor.body[n - 2]);
        let (mut quo, mut rem) = div_ww_w(&h1, &h2, &d1);
        while quo == u64::MAX as u128 + 1
            || quo * (h3 as u128) > (u64::MAX as u128 + 1) * rem + d2 as u128
        {
            quo -= 1;
            rem += d1 as u128;
            if rem >= (u64::MAX as u128 + 1) {
                break;
            }
        }
        let b: bool;
        let mut borrow: u128 = 0;
        for z in 0..n {
            let p = quo * divisor.body[z] as u128 + borrow;
            let (res, b) = (dividend.body[i + z]).overflowing_sub(p as u64);
            borrow = (p >> 64) + if b { 1 } else { 0 };
            dividend.body[i + z] = res as u64
        }
        (dividend.body[i + n], b) = dividend.body[i + n].overflowing_sub(borrow as u64);
        if b {
            quo -= 1;
            let mut carry: bool = false;
            for z in 0..n {
                (dividend.body[i + z], carry) =
                    dividend.body[i + z].carrying_add(divisor.body[z], carry)
            }
            if carry {
                (dividend.body[i + n], _) = dividend.body[i + n].carrying_add(0, carry)
            }
        }
        result[i] = quo as u64
    }
    dividend >>= ld;
    dividend.body = trim(&mut dividend.body);
    result = trim(&mut result);
    (result, dividend.body) //quo,rem
}
fn sh_div_abs(v1: &BigInt, v2: &BigInt) -> (Vec<u64>, Vec<u64>) {
    let (mut rem, len) = (0u128, v1.body.len());
    let mut quo: u128;
    let mut result: Vec<u64> = vec![0; len];
    for (a, b) in zip(v1.body.iter().rev(), result.iter_mut().rev()) {
        (quo, rem) = div_ww_w(&(rem as u64), a, &v2.body[0]);
        *b = quo as u64
    }
    result = trim(&mut result);
    return (result, vec![rem as u64]); //quo,rem
}
fn div_1_1_abs(v1: &BigInt, v2: &BigInt) -> (Vec<u64>, Vec<u64>) {
    return (vec![v1.body[0] / v2.body[0]], vec![v1.body[0] % v2.body[0]]);
}
fn div_sing(sing1: bool, sing2: bool, method: bool) -> bool {
    match (sing1, sing2, method) {
        (true, true, true) => false,
        (false, false, true) => false,
        (false, true, true) => true,
        (true, false, true) => true,
        (true, _, false) => true,
        (false, _, false) => false,
    }
}
fn div_man(v1: &BigInt, v2: &BigInt) -> (Vec<u64>, Vec<u64>) {
    let (l1, l2) = (v1.body.len(), v2.body.len());
    match (l1, l2) {
        (1, 1) => div_1_1_abs(v1, v2),
        (a, 1) if a > 1 => sh_div_abs(v1, v2),
        (a, b) if a > 1 && b > 1 => div_abs(v1, v2),
        _ => div_abs(v1, v2),
    }
}
fn shr_ass(v1: &mut BigInt, other: &u32) {
    let shift = other % 64;
    let drain = other / 64;
    if drain != 0 {
        v1.body.drain(0..min(drain as usize, v1.body.len()));
        if v1.body.len() == 0 {
            v1.body = vec![0];
            return;
        }
    }
    if shift == 0 {
        return;
    }
    let mut carry = 0u64;
    for a in v1.body.iter_mut().rev() {
        let temp = *a;
        *a = (*a >> shift) | carry;
        carry = temp << (64 - shift);
    }
    v1.body = trim(&mut v1.body);
}
fn shr(v1: &BigInt, other: &u32) -> BigInt {
    let mut result = BigInt {
        neg: v1.neg,
        body: v1.body.clone(),
    };
    let shift = other % 64;
    let drain = other / 64;
    if drain != 0 {
        result.body.drain(0..min(drain as usize, result.body.len()));
        if result.body.len() == 0 {
            result.body = vec![0];
            return result;
        }
    }
    if shift == 0 {
        return result;
    }
    let mut carry = 0u64;
    for a in result.body.iter_mut().rev() {
        let temp = *a;
        *a = (*a >> shift) | carry;
        carry = temp << (64 - shift);
    }
    result.body = trim(&mut result.body);
    result
}
fn shl_ass(v1: &mut BigInt, other: &u32) {
    if *other == 0 {
        return;
    }
    let zeros = other / 64;
    let shift = other % 64;
    if zeros != 0 {
        let app: Vec<u64> = vec![0; zeros as usize];
        v1.body.splice(0..0, app);
    }
    if shift == 0 {
        return;
    }
    let mut carry = 0u64;
    for a in v1.body.iter_mut() {
        let temp = (*a as u128) << shift;
        *a = (temp as u64) | carry;
        carry = (temp >> 64) as u64;
    }
    if carry != 0 {
        v1.body.push(carry)
    }
}
fn shl(v1: &BigInt, other: &u32) -> BigInt {
    if other == &0 {
        return v1.clone();
    }
    let mut result = BigInt {
        neg: v1.neg,
        body: v1.body.clone(),
    };
    let zeros = other / 64;
    let shift = other % 64;
    if zeros != 0 {
        let app: Vec<u64> = vec![0; zeros as usize];
        result.body.splice(0..0, app);
    }
    if shift == 0 {
        return v1.clone();
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
