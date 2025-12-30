use crate::int::int::*;
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
pub(crate) static PRECISION: AtomicU64 = AtomicU64::new(4);
impl BigFloat {
    #[allow(unused)]
    fn precision(value: u64) {
        PRECISION.store(value + 2, Ordering::Relaxed);
    }
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
        self.exp += change;
        self.body <<= 64 * change;
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
#[allow(unused)]
pub fn cbrt(v1: &BigFloat) -> BigFloat {
    let mut guess: BigFloat = BigFloat::from(pow10(&((v1.body.len() / 3) as u64)));
    let (mut current, target) = (1 as u64, PRECISION.load(Ordering::Relaxed) * 64 as u64);
    let two = BigFloat {
        body: BigInt::from(2),
        exp: 0,
    };
    let x2 = &two * v1;
    while current < target {
        let x1 = &guess * &guess * &guess;

        guess = &guess * &((&x1 + &x2) / ((&two * &x1) + v1));
        current *= 3
    }
    guess
}
#[allow(unused)]
pub fn sqrt(v1: &BigFloat) -> BigFloat {
    let mut guess: BigFloat = BigFloat::from(pow10(&((v1.body.len() / 2) as u64)));
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
