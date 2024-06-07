use num::{One, Zero};
use num::BigInt;
use modpow::*;
use rand::Rng;

pub fn is_prime(n: u64, k: usize) -> bool {
    if k > 100 {
        panic!("Probability value must be between 0 to 100, got {}.", k);
    }
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 {
        return false;
    }

    // Write n − 1 as 2^s·d with d odd by factoring powers of 2 from n − 1
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a: u32 = rng.gen_range(2..u32::try_from(n).unwrap() - 1);
        let mut x = modpow(&a.into(), &d, &n);

        let tmp: num::bigint::BigInt = n.try_into().unwrap();
        if x == num::bigint::BigInt::one() || x == tmp.clone() - num::bigint::BigInt::one() {
            continue;
        }

        let mut composite = true;
        for _ in 1..s {
            x = modpow(&x, &num::bigint::BigInt::from(2), &n);

            if x == tmp.clone() - num::bigint::BigInt::one() {
                composite = false;
                break;
            }
        }

        if composite {
            return false;
        }
    }

    true
}

pub fn get_prime() -> u64 {
    let mut rng = rand::thread_rng();
    let mut num: u32 = rng.gen();

    while !is_prime(num.into(), 5) {
        num = rng.gen();
    }
    num.into()
}

pub fn generate_prime() -> (u64, u64) {
    let mut p: u64;
    let mut q: u64;

    loop {
        //p += 1;
        p = get_prime();
        q = get_prime();

        print!(".");
        let modules: u64 = p * q;

        if modules >> 63 == 1 {
            println!("+++++");
            break;
        }
    }
    println!("new line");
    (p.into(), q.into())
}

pub fn gcd(a: &num::bigint::BigInt, b: &num::bigint::BigInt) -> num::bigint::BigInt {
    if b == &num::bigint::BigInt::zero() {
        return a.clone();
    }
    return gcd(b, &(a % b));
}

pub fn lcm(a: &num::bigint::BigInt, b: &num::bigint::BigInt) -> num::bigint::BigInt {
    if b == &num::bigint::BigInt::zero() {
        return a.clone();
    }
    return (a * b) / gcd(a, b);
}

pub fn mod_inverse(mut a: num::bigint::BigInt, mut b: num::bigint::BigInt) -> num::bigint::BigInt {
    let m = b.clone();
    let mut y = num::bigint::BigInt::zero();
    let mut x = num::bigint::BigInt::one();

    if b == num::bigint::BigInt::one() {
        return num::bigint::BigInt::zero();
    }
    while a > num::bigint::BigInt::one() {
        let q = a.clone() / b.clone();
        let mut t = b.clone();

        b = a % b.clone();
        a = t;
        t = y.clone();

        y = x - q * y;
        x = t;
    }
    if x < num::bigint::BigInt::zero() {
        x += m;
    }
    return x;
}

fn int_to_bytes(mut num: u64) -> Vec<u8> {
    let mut bytes = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let mut i = 7;

    while num > 0 {
        let tmp = num % 256;
        bytes[i] = tmp as u8;
        num /= 256;
        i -= 1;
    }
    bytes
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mod_inverse() {
        let a = num::bigint::BigInt::from(17);
        let b = num::bigint::BigInt::from(780);
        let result = mod_inverse(a, b);
        assert_eq!(result, num::bigint::BigInt::from(413));
    }

    #[test]
    fn test_int_to_bytes() {
        let num: u64 = 65537;
        let result = int_to_bytes(num);
        println!("vec: {:?} {:?}", result, num.to_be_bytes());
        // assert!(result, num.to_be_bytes());
    }
}

#[test]
fn test_some() {
    let p = BigInt::from(2);
    let q = BigInt::from(7);
    let n = p.clone() * q.clone();
    let e = lcm(&(p.clone() - BigInt::one()), &(q.clone() - 1));
    println!("n = {}, and e = {}", n, e);
}