#[allow(unused)]
#[allow(unused)]
use crate::int::int::{BigInt, pow, pow10};
use std::{thread::sleep, time::Duration};
mod int;
fn main() {
    let mut l1 = BigInt::from("2");
    l1 <<= 68;
    println!("shifted {:?}", l1);
    l1 >>= 67;
    println!("shifted {:?}", l1);
    let mut l2 = BigInt::from("2^64");
    l2 += BigInt::from(10);
    l1 = l1 / l2;
    println!("result {:?}", l1);
    sleep(Duration::from_secs(1));
    for _ in 0..0 {
        println!("Start");
        let c1 = std::time::Instant::now();
        let c2 = c1.elapsed().as_micros();
        println!("result {:?}", l1);
        println!("time {}  mcs", c2);
    }
}
