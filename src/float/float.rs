use crate::int::int::*;
use std::cmp::*;

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Debug, Clone)]
pub struct BigFloat {
    pub(crate) body: BigInt,
    pub(crate) exp: u64,
}
#[allow(unused)]
static E: OnceLock<BigFloat> = OnceLock::new();
#[allow(unused)]
static PI: OnceLock<BigFloat> = OnceLock::new();
pub static PRECISION: AtomicU64 = AtomicU64::new(7);
pub static ROUNDING: AtomicUsize = AtomicUsize::new(1);
impl BigFloat {
    fn round(&mut self, last: &mut Vec<u64>) {
        match ROUNDING.load(Ordering::Relaxed) {
            1 => {
                for i in 0..last.len() - 1 {
                    if last[i] > 9223372036854775808 {
                        last[i + 1] += 1
                    }
                }
                if last[last.len() - 1] > 9223372036854775808 {
                    self.body += BigInt::from(1);
                }
                self
            }
            2 => {
                self.body += BigInt::from(1);
                self
            }
            _ => self,
        };
    }
    pub fn pow(&self, power: &u64) -> BigFloat {
        if power < &0 {
            return BigFloat::default();
        }
        if BigFloat::from(0) == self {
            return BigFloat::default();
        } else if BigFloat::from(1) == *self || power == &0 {
            return BigFloat::from(1);
        } else if power == &1 {
            return self.clone();
        }
        let mut res = BigFloat {
            body: BigInt::from(1),
            exp: 0,
        };
        if power % 2 == 1 && self.body.neg {
            res.body.neg = true
        }
        let mut b: BigFloat = self.clone();
        b.body.neg = false;
        b.normalize();
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
    pub(crate) fn normalize(&mut self) {
        let v = PRECISION.load(Ordering::Relaxed);
        if self.exp != v {
            if v > self.exp {
                self.body <<= 64 * (v - self.exp);
            } else {
                let mut last = self
                    .body
                    .body
                    .get(0..v as usize)
                    .unwrap_or(&[0u64])
                    .to_owned();
                self.body >>= 64 * (self.exp - v);
                self.round(&mut last);
            }
        };
        if ROUNDING.load(Ordering::Relaxed) == 2 {
            self.body += BigInt::from(1);
        }
        self.exp = v;
    }
    pub(crate) fn denormalize(&mut self, change: u64) {
        let v = self.exp;
        self.exp = change;
        if v != self.exp {
            if v < self.exp {
                self.body <<= 64 * (self.exp - v);
            } else {
                self.body >>= 64 * (v - self.exp);
            }
        };
    }
}
#[allow(unused)]
pub fn precision(value: u64) {
    PRECISION.store(value + 2, Ordering::Relaxed);
}
pub fn round_mod(value: usize) {
    if value == 0 || value == 1 || value == 2 {
        ROUNDING.store(value, Ordering::Relaxed);
    }
}
#[allow(unused)]
fn calc_pi(precision: u32) -> BigFloat {
    todo!("d")
}
#[allow(unused)]
fn calc_e(precision: u32) -> BigFloat {
    todo!("")
}
pub fn root(v1: &BigFloat, n: usize) -> BigFloat {
    let (prec, round) = (
        PRECISION.load(Ordering::Relaxed) - 2,
        ROUNDING.load(Ordering::Relaxed),
    );
    precision(prec + ((max(v1.body.len(), 30) / 2) * n) as u64);
    round_mod(2);
    let mut guess: BigFloat = BigFloat::from(
        BigInt::from(1) << &(((max(v1.body.len() - v1.exp as usize, n as usize) / n) * 64) as u64),
    );
    let mut val = v1.clone();
    val.normalize();
    let v = BigFloat {
        body: BigInt::from(n - 1),
        exp: 0,
    };
    let mut count = (((PRECISION.load(Ordering::Relaxed) * 64).ilog2() + 1) * 3) as i32;
    let div = BigFloat::from(1) / BigFloat::from(n as u128);
    let (mut near, mut last) = (
        false,
        guess
            .body
            .body
            .get(((guess.exp - 1) as usize)..)
            .unwrap_or(&[0u64])
            .to_owned(),
    );
    while !near {
        if last
            == *guess
                .body
                .body
                .get(((guess.exp) as usize)..)
                .unwrap_or(&[0u64])
        {
            count -= 1;
            if count < 0 {
                near = true
            }
        } else {
            last = guess
                .body
                .body
                .get(((guess.exp) as usize)..)
                .unwrap_or(&[0u64])
                .to_owned()
        }
        let x1: BigFloat = guess.pow(&((n - 1) as u64));
        guess = &div * ((&v * &guess) + (&val / &x1));
    }
    round_mod(round);
    precision(prec);
    guess.normalize();
    guess
}
