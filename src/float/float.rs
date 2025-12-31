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
pub static PRECISION: AtomicU64 = AtomicU64::new(12);
impl BigFloat {
    pub(crate) fn normalize(&mut self) {
        let v = PRECISION.load(Ordering::Relaxed);
        if self.exp != v {
            if self.exp < v {
                self.body <<= 64 * (v - self.exp);
            } else {
                self.body >>= 64 * (self.exp - v);
            }
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
pub fn cbrt(v1: &BigFloat) -> BigFloat {
    let prec = PRECISION.load(Ordering::Relaxed) - 2;
    precision(prec * 3);
    let prec1 = PRECISION.load(Ordering::Relaxed);
    let mut guess: BigFloat = BigFloat::from(pow(
        &BigInt::from(2),
        &((max((v1.body.len() - v1.exp as usize), 3) / 3 * 16) as u64),
    ));
    let mut val = v1.clone();
    val.normalize();
    let (mut current, target) = (1 as f64, (PRECISION.load(Ordering::Relaxed) * 64) as f64);
    let two = BigFloat {
        body: BigInt::from(2),
        exp: 0,
    };
    let x2 = &two * &val;
    while current < target {
        let mut x1: BigFloat = BigFloat {
            body: &guess.body * &guess.body * &guess.body,
            exp: prec1 * 3,
        };
        x1.normalize();
        let b = &((&x1 + &x2) / ((&two * &x1) + &val));
        guess = &guess * b;
        current *= 1.25
    }
    precision(prec);
    guess.normalize();
    guess
}
#[allow(unused)]
pub fn sqrt(v1: &BigFloat) -> BigFloat {
    let mut guess: BigFloat = BigFloat::from(pow(
        &BigInt::from(2),
        &(((v1.body.len() - v1.exp as usize) / 2 * 32) as u64),
    ));
    let (mut current, target) = (1 as u64, PRECISION.load(Ordering::Relaxed) * 64 as u64);
    let two = BigFloat {
        body: BigInt::from(2),
        exp: 0,
    };
    while current < target {
        let x2 = &guess;
        let x1 = &guess * &guess;
        guess = (&x1 + v1) / (x2 * &two);
        current *= 2
    }
    guess
}
