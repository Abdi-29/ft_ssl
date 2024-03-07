use num::{One, Zero};
use num::bigint::BigInt;
use rand::Rng;
use modpow::*;
// use num_bigint::BigInt;

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

        let tmp: BigInt = n.try_into().unwrap();
        if x == BigInt::one() || x == tmp.clone() - BigInt::one() {
            continue;
        }

        let mut composite = true;
        for _ in 1..s {
            x = modpow(&x, &BigInt::from(2), &n);

            if x == tmp.clone() - BigInt::one() {
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

pub fn generate_prime() -> u64 {
    let mut rng = rand::thread_rng();
    let mut p: u32 = rng.gen();

    while !is_prime(p.into(), 5) {
        p += 1;
    }
    p.into()
}

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt{
    if b == &BigInt::zero() {
        return a.clone();
    }
    return gcd(b, &(a % b));
}

pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    if b == &BigInt::zero() {
        return a.clone();
    }
    return (a * b) / gcd(a, b);
}

pub fn mod_inverse(mut a: BigInt, mut b: BigInt) -> BigInt {
    let m = b.clone();
    let mut y  = BigInt::zero();
    let mut x = BigInt::one();

    if b == BigInt::one() {
        return BigInt::zero()
    }
    while a > BigInt::one() {
        let q = a.clone() / b.clone();
        let mut t = b.clone();

        b = a % b.clone();
        a = t;
        t = y.clone();

        y = x - q * y;
        x = t;
    }
    if x < BigInt::zero() {
        x += m;
    }
    return x;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mod_inverse() {
        let a = BigInt::from(17);
        let b = BigInt::from(780);
        let result = mod_inverse(a, b);
        assert_eq!(result, BigInt::from(413));
    }
}