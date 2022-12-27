use std::{str::FromStr, ops::{Sub, Mul}};
use num_bigint::{BigInt, Sign};
use num::Integer;
use num::One;
use num::Zero;
// use crypto_bigint::{};
#[derive(Debug)]
struct E521 {
    pub x: BigInt,
    pub y: BigInt,
    pub p: BigInt,
    pub d: BigInt,
    pub r: BigInt,
    pub n: BigInt,
}

fn main() {
    println!("gen point y coord: {:?}", get_e521_gen_point(false).y);
    // println!("id point: {:?}", get_e521_id_point());
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

fn get_d() -> BigInt { return Some(BigInt::from(-376014)).unwrap();}



// Generates the neutral point (0, 1)
fn get_e521_id_point() -> E521 {
    let point = E521{
        x: BigInt::from(0),
        y: BigInt::from(1),
        p: get_p(),
        d: get_d(),
        r: get_r(), 
        n: get_n()
    };
    point
}

// Gets point for arbitrary (x, y) TODO verify point is on curve
fn get_e521_point(x: BigInt, y: BigInt) -> E521 {
    let point = E521{
        x: x,
        y: y,
        p: get_p(),
        d: get_d(),
        r: get_r(), 
        n: get_n()
    };
    point
}

// Gets point for arbitrary (x, y) TODO verify point is on curve
fn get_e521_gen_point(lsb: bool) -> E521 {
    let x = BigInt::from(4);
    let new_x = x.clone();
    let point = E521{
        x: x,
        y: solve_for_y(&new_x, get_p(), lsb),
        p: get_p(),
        d: get_d(),
        r: get_r(), 
        n: get_n()
    };
    point
}

fn solve_for_y(x: &BigInt, p: BigInt, msb: bool) -> BigInt {
    let num = BigInt::from(1) - x.pow(2);
    let num = mod_formula(&num, &p);
    let denom = BigInt::from(376_014) * x.pow(2) + BigInt::from(1);
    let denom = mod_formula(&denom, &p);
    let denom = mod_inv(&denom, &p);
    let radicand = num * denom;
    let y = sqrt(&radicand, msb);
    y
}

fn mod_formula(a: &BigInt, b: &BigInt) -> BigInt {
    ((a % b) + b) % b
}

fn mod_inv(n: &BigInt, p: &BigInt) -> BigInt {
    if p.is_one() { return BigInt::one() }

    let (mut a, mut m, mut x, mut inv) = (n.clone(), p.clone(), BigInt::zero(), BigInt::one());
    while a < BigInt::zero() { a += p }
    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }
 
    if inv < BigInt::zero() { inv += p }
    inv
}

fn sqrt(v: &BigInt, lsb: bool) -> BigInt {
    if v.sign() ==  Sign::NoSign{
        return BigInt::from(0);
    }
    let p = get_p();
    let r = v.modpow(&((p.clone() >> 2) + 1), &p);
    if !r.bit(0).eq(&lsb) {
        let new_r = &p - r; // correct the lsb
        let newr2 = new_r.clone();
        let return_r = new_r.clone();
        let bi = mod_formula(&new_r.mul(newr2).sub(v), &p);
        if bi.sign() == Sign::NoSign {
            return return_r;
        } else {
            return BigInt::from(0);
        }
    }
    r
}
