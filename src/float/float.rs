use crate::int::int::*;
use std::cmp::*;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

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
impl BigFloat {
    pub fn pow(&self, power: &u64) -> BigFloat {
        if power < &0 {
            return BigFloat::default();
        }
        if BigInt::from(0) == self {
            return BigFloat::default();
        } else if BigFloat::from(1) == *self || power == &0 {
            return BigFloat::from(1);
        } else if power == &1 {
            return *self;
        }
        let mut res = BigFloat {
            body: BigInt::from(1),
            exp: 0,
        };
        if power % 2 == 1 {
            res.body.neg = true
        }
        let mut b: BigFloat = BigFloat::from(self.body);
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
            if self.exp < v {
                self.body <<= 64 * (v - self.exp);
            } else {
                if self.body.body.get(v as usize).unwrap_or(&0) > &9223372036854775808u64 {
                    self.body >>= 64 * (self.exp - v);
                    self.body += BigInt::from(1)
                } else {
                    self.body >>= 64 * (self.exp - v);
                }
            }
        };
        *self += BigFloat {
            body: BigInt::from(1),
            exp: 0,
        };
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
fn precision(value: u64) {
    PRECISION.store(value + 2, Ordering::Relaxed);
}
#[allow(unused)]
fn calc_pi(precision: u32) -> BigFloat {
    todo!("d")
}
#[allow(unused)]
fn calc_e(precision: u32) -> BigFloat {
    todo!("")
}
#[allow(unused)]
pub fn root(v1: &BigFloat, n: usize) -> BigFloat {
    let prec = PRECISION.load(Ordering::Relaxed) - 2;
    precision(prec * 3);
    let mut guess: BigFloat = BigFloat::from(
        (BigInt::from(1)
            << &(((max((v1.body.len() - v1.exp as usize), n as usize) / n) * 64) as u64)),
    );
    let mut val = v1.clone();
    val.normalize();
    let (mut current, target) = (1 as f64, (PRECISION.load(Ordering::Relaxed) * 64) as f64);
    let v = BigFloat {
        body: BigInt::from(n),
        exp: 0,
    };
    let div = BigFloat::from(1) / BigFloat::from(n as u128);
    while current < target {
        let mut x1: BigFloat = guess.pow(&((n - 1) as u64));
        guess = &div * ((&v * &guess) + (&val / &x1));
        current *= 1.1
    }
    precision(prec);
    guess.normalize();
    guess
}
