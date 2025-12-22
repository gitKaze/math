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
#[derive(Debug)]
pub struct BigInt {
    pub neg: bool,
    pub body: Vec<u64>,
}
impl BigInt {
    #[inline(always)]
    #[allow(dead_code)]
    fn sub_abs(&self, _other: &Self) -> Vec<u64> {
        todo!("d")
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
    //if base == BigInt::from(0) {
    //    return BigInt::default();
    //} else if base == BigInt::from(1) || power == 0 {
    //    return BigInt::from(1);
    //} else if power == 1 {
    //    return BigInt {
    //        neg: (false),
    //        body: (base.body.clone()),
    //    };
    //}
    let mut res = BigInt::from(1);
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
