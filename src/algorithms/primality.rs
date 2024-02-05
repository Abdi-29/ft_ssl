use rand::Rng;
use modpow::*;

use num::bigint::BigInt;

pub fn is_prime(n:u64, k: usize) -> bool {
    if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    //write n − 1 as 2s·d with d odd by factoring powers of 2 from n − 1
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    println!("testing {}", s);
    let mut rng = rand::thread_rng();
    
    for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = modpow(&a, &d, &n);
        if x == BigInt::from(1) || x == BigInt::from(n - 1) {
            return false;
        }
        for _ in 1..s {
            x = modpow(&x, &BigInt::from(2), &n);
            if x == BigInt::from(n - 1) {
                return false;
            }
        }
    }
    true
}