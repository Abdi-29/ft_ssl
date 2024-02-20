use crate::primality::generate_prime;

pub struct RsaKey {
    pub modulus: bigUint,
    pub public_exponent: bigUint,
    pub private_exponent: bigUint
}

pub fn generate_rsa_key() -> RsaKey {
    //Choose two large prime numbers p and q. 
    let p = generate_prime();
    let q = generate_prime();

    //Compute n = pq
    let modulus = &p * &q;

    /*
        Compute λ(n), where λ is Carmichael's totient function.
        Since n = pq, λ(n) = lcm(λ(p), λ(q)), and since p and q are prime, 
        λ(p) = φ(p) = p − 1, and likewise λ(q) = q − 1.
        Hence λ(n) = lcm(p − 1, q − 1). 
        18446744073709551615 u64
        9223372036854775807 bigUint
    */
    let totient = lcm(p, q);
    let public_exponent = 65537;
    let private_exponent = public_exponent.modpow(u64::one(), totient);
    RsaKey {
        modulus,
        public_exponent,
        private_exponent
    }
}