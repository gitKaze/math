use crate::int::int::*;
use std::fmt;
use std::ops::*;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
include!("ops.rs");
#[derive(Debug, Clone)]
pub struct BigFloat {
    body: BigInt,
    exp: u32,
}
#[allow(unused)]
static E: OnceLock<BigFloat> = OnceLock::new();
#[allow(unused)]
static PI: OnceLock<BigFloat> = OnceLock::new();
static PRECISION: AtomicU32 = AtomicU32::new(20);
impl BigFloat {
    #[allow(unused)]
    fn precision(value: u32) {
        PRECISION.store(value, Ordering::Relaxed);
    }
    fn normalize(&mut self) {
        let v = PRECISION.load(Ordering::Relaxed);
        if self.exp != v {
            if self.exp < v {
                self.body <<= 64 * (v - self.exp) as u32;
            } else {
                self.body >>= 64 * (self.exp - v) as u32;
            }
        };
        self.exp = v;
    }
    fn denormalize(&mut self, change: i32) {
        let mut v = self.exp as i32 + change;
        if v <= 0 {
            panic!("negative or 0 exp")
        } else {
            v = v.abs();
        }
        let b = v as u32;
        if self.exp < b {
            self.body <<= 64 * (b - self.exp) as u32;
        } else {
            self.body >>= 64 * (self.exp - b) as u32;
        }
        self.exp = b;
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
