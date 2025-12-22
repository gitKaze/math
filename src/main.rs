#[allow(unused)]
#[allow(unused)]
use crate::int::int::{BigInt, pow, pow10};
use std::{thread::sleep, time::Duration};

mod int;
fn main() {
    let mut l1 = BigInt::from("3");
    let l2 = 1280000;
    sleep(Duration::from_secs(1));
    for _ in 0..1 {
        println!("Start");
        let c1 = std::time::Instant::now();
        l1 = pow(&l1, &l2);
        let c2 = c1.elapsed().as_micros();
        println!("result {:?}", l1.body.len());
        println!("time {}  mcs", c2);
    }
}
