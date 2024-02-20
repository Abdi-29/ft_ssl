use rand::Rng;
use modpow::*;

use num_bigint::BigInt;

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
        let a = rng.gen_range(2..n - 1);
        let mut x = modpow(&a, &d, &n);

        if x == BigInt::from(1) || x == BigInt::from(n - 1) {
            continue;
        }

        let mut composite = true;
        for _ in 1..s {
            x = modpow(&x, &BigInt::from(2), &n);

            if x == BigInt::from(n - 1) {
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
    let mut p: u64 = rng.gen();
    // let mut q: u64 = rng.gen();

    while !is_prime(p, 5) {
        p += 1;
    }
    p
}

pub fn gcd(a: u64, b: u64) -> u64{
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return (a * b) / gcd(a, b);
}
