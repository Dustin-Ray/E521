use std::{str::FromStr, ops::{Sub}};
use num_bigint::{BigInt};

struct e521 {
    x: BigInt,
    y: BigInt,
    p: BigInt,
    d: BigInt,
    r: BigInt,
    n: BigInt,
}

fn main() {
    println!("value of r: {:?}", get_r());
    println!("value of p: {:?}", get_p());
    println!("value of n: {:?}", get_n());
    println!("value of d: {:?}", get_d());

}

fn get_r() -> BigInt {
    let r = BigInt::from(2);
    let r = r.pow(519);
    let s = BigInt::from_str("337554763258501705789107630418782636071904961214051226618635150085779108655765").unwrap();
    let r = r.sub(s);
    return r;
}

fn get_p() -> BigInt {
    let p = BigInt::from(2);
    let p = p.pow(521);
    let p = p.sub(1);
    return p;
}

fn get_n() -> BigInt {

    let n = get_r();
    let n = n.checked_mul(&BigInt::from(4)).unwrap();
    return n;
}

fn get_d() -> BigInt {

    let d = BigInt::from(-376014);
    return d;

}