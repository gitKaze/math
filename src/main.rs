#[allow(unused)]
#[allow(unused)]
use crate::float::float::*;
#[allow(unused)]
use crate::int::int::*;
use std::time::Instant;
mod float;
mod int;
#[allow(unused)]
fn test(i1: &str, i2: &str) {
    let (v1, v2, v3, v4) = (
        BigInt::from(i1),
        BigInt::from(i2),
        BigFloat::from(i1),
        BigFloat::from(i2),
    );
    println!("BigInt:");
    println!("values {} | {}", v1, v2);
    println!("multiplication:");
    let t = Instant::now();
    let temp = &v1 * &v2;
    let end = Instant::elapsed(&t);
    println!("result {} completed in {:?}", temp, end);
    println!("addition:");
    let t = Instant::now();
    let temp = &v1 + &v2;
    let end = Instant::elapsed(&t);
    println!("result {} completed in {:?}", temp, end);
    println!("substraction:");
    let t = Instant::now();
    let temp = &v1 - &v2;
    let end = Instant::elapsed(&t);
    println!("result {} completed in {:?}", temp, end);
    println!("division:");
    let t = Instant::now();
    let temp = &v1 / &v2;
    let end = Instant::elapsed(&t);
    println!("result {} completed in {:?}", temp, end);
    println!("remainder:");
    let t = Instant::now();
    let temp = &v1 % &v2;
    let end = Instant::elapsed(&t);
    println!("result {} completed in {:?}\n\n\n", temp, end);
    println!("BigFloat with  precition of {}:", v3.exp - 2);
    println!("values {} | {}", v3, v4);
    println!("{:?} | {:?}", v3, v4);
    println!("multiplication:");
    let t = Instant::now();
    let temp = &v3 * &v4;
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("addition:");
    let t = Instant::now();
    let temp = &v3 + &v4;
    let end = Instant::elapsed(&t);
    println!("{:?}", temp);
    println!("result {} completed in {:?}", temp, end);
    println!("substraction:");
    let t = Instant::now();
    let temp = &v3 - &v4;
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("division:");
    let t = Instant::now();
    let temp = &v3 / &v4;
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("square root:");
    let t = Instant::now();
    let temp = root(&v3, 2);
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("cube root:");
    let t = Instant::now();
    let temp = root(&v3, 3);
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("square root:");
    let t = Instant::now();
    let temp = root(&v4, 2);
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
    println!("cube root:");
    let t = Instant::now();
    let temp = root(&v4, 3);
    let end = Instant::elapsed(&t);

    println!("result {} completed in {:?}", temp, end);
}
fn main() {
    println!(
        "{}",
        (BigFloat::from(1) / BigFloat::from(3)) * BigFloat::from("2^64")
    );
    let (i1, i2) = ("2^128", "2^19683");
    #[allow(unused)]
    let (v1, v2, v3, v4) = (
        BigInt::from(i1),
        BigInt::from(i2),
        BigFloat::from(i1),
        BigFloat::from(i2),
    );
    test(i1, i2);
}
