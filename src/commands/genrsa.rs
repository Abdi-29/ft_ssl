use num::{BigInt, BigUint, One};

use crate::generate_prime;
use crate::algorithms::primality::{lcm, mod_inverse};
// use modpow::*;

pub struct RsaKey {
    pub modulus: u64,
    pub public_exponent: u64,
    pub private_exponent: u64
}

pub fn generate_rsa_key() -> BigInt {
    /*Choose two large prime numbers p and q.
        step 1
    */
    let p = generate_prime();
    let q = generate_prime();

    //Compute n = pq -> step 2
    let modulus: u64 = p * q;
    println!("p: {}, q: {} and modulus: {}", p, q, modulus);


    /*
        Compute λ(n), where λ is Carmichael's totient function.
        Since n = pq, λ(n) = lcm(λ(p), λ(q)), and since p and q are prime, 
        λ(p) = φ(p) = p − 1, and likewise λ(q) = q − 1.
        Hence λ(n) = lcm(p − 1, q − 1). 
        18446744073709551615 u64
        9223372036854775807 bigUint
    */
    let a: BigInt = p.try_into().unwrap();
    let b: BigInt = q.try_into().unwrap();
    let totient = lcm(&(a - BigInt::one()), &(b - 1));
    let public_exponent: BigInt = BigInt::from(65537);
    let private_exponent = mod_inverse(public_exponent, totient.clone().try_into().unwrap());
    println!("private_exponent: {}", private_exponent);
    // let private_exponent: BigUint = 
    //9_223_372_036_854_775_807
    //170_141_183_460_469_231_731_687_303_715_884_105_727
   
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