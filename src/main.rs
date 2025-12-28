#[allow(unused)]
#[allow(unused)]
use crate::float::float::*;
use crate::int::int::*;
use std::{thread::sleep, time::Duration};
mod float;
mod int;
fn main() {
    let l1 = BigInt::from("2^9876");
    println!("res {}", l1);
    let _l2 = BigInt::from("-45");
    println!("result {}", l1);
    sleep(Duration::from_secs(1));
    for _ in 0..0 {
        println!("Start");
        let c1 = std::time::Instant::now();
        let c2 = c1.elapsed().as_micros();
        println!("result {:?}", l1);
        println!("time {}  mcs", c2);
    }
}
