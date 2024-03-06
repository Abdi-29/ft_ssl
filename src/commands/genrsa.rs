use num::{BigUint, One};

use crate::generate_prime;
use crate::algorithms::primality::lcm;
// use modpow::*;

pub struct RsaKey {
    pub modulus: u64,
    pub public_exponent: u64,
    pub private_exponent: u64
}

pub fn generate_rsa_key() -> BigUint {
    /*Choose two large prime numbers p and q.
        step 1
    */
    let p = &BigUint::from(generate_prime());
    let q = &BigUint::from(generate_prime());

    //Compute n = pq -> step 2
    let modulus: BigUint = p * q;
    println!("p: {}, q: {} and modulus: {}", p, q, modulus);


    /*
        Compute λ(n), where λ is Carmichael's totient function.
        Since n = pq, λ(n) = lcm(λ(p), λ(q)), and since p and q are prime, 
        λ(p) = φ(p) = p − 1, and likewise λ(q) = q − 1.
        Hence λ(n) = lcm(p − 1, q − 1). 
        18446744073709551615 u64
        9223372036854775807 bigUint
    */

    let totient = lcm(&(p - BigUint::one()), &(q - BigUint::one()));
    // let public_exponent: BigUint = 65537;
    // let private_exponent: BigUint = 
   
    // RsaKey {
    //     modulus,
    //     public_exponent,
    //     private_exponent
    // }
    totient
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_generate_rsa_key() {
        let test = generate_rsa_key();
        println!("result: {}", test);
    }
}