use num::{One, BigInt};
// use num_num::bigint::BigInt::c;
// use num::num::bigint::BigInt::num::bigint::BigInt;

use crate::generate_prime;
use crate::algorithms::primality::{lcm, mod_inverse};
// use modpow::*;

/*
The content of the RSA private key is as follows:

-----BEGIN RSA PRIVATE KEY-----
RSAPrivateKey ::= SEQUENCE {
  version           Version,
  modulus           INTEGER,  -- n
  publicExponent    INTEGER,  -- e
  privateExponent   INTEGER,  -- d
  prime1            INTEGER,  -- p
  prime2            INTEGER,  -- q
  exponent1         INTEGER,  -- d mod (p-1)
  exponent2         INTEGER,  -- d mod (q-1)
  coefficient       INTEGER,  -- (inverse of q) mod p
  otherPrimeInfos   OtherPrimeInfos OPTIONAL
}
-----END RSA PRIVATE KEY-----

while a RSA public key contains only the following data:

-----BEGIN RSA PUBLIC KEY-----
RSAPublicKey ::= SEQUENCE {
    modulus           INTEGER,  -- n
    publicExponent    INTEGER   -- e
}
-----END RSA PUBLIC KEY-----

*/
pub struct RsaKey {
    pub modulus: BigInt,
    pub public_exponent: BigInt,
    pub private_exponent: BigInt,
    pub prime: [BigInt; 2],
    pub exponent: [BigInt; 2],
    pub coefficient: BigInt,
}

pub fn generate_rsa_key() -> BigInt {
    /*Choose two large prime numbers p and q.
        step 1
    */
    let (p, q) = generate_prime();

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
    let totient = lcm(&(a.clone() - BigInt::one()), &(b.clone() - 1));
    let modulus: BigInt = a.clone() * b.clone();
    let public_exponent: BigInt = 65537.into();
    let private_exponent: BigInt = mod_inverse(public_exponent.clone(), totient.clone().try_into().unwrap());
    let coefficient: BigInt = mod_inverse(b.clone(), a.clone());
    RsaKey {
        modulus,
        public_exponent: public_exponent.clone(),
        private_exponent: private_exponent.clone(),
        prime: [a.clone(), b.clone()],
        exponent: [private_exponent.clone() % (a.clone() - BigInt::one()), private_exponent.clone() % (b.clone() - BigInt::one())],
        coefficient
    };
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