use num::{One, BigInt};
// use num_num::bigint::BigInt::c;
// use num::num::bigint::BigInt::num::bigint::BigInt;
use base64;
use crate::generate_prime;
use crate::algorithms::primality::{lcm, mod_inverse};

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

    totient
}

pub fn encode_integer(der_encoding: &mut Vec<u8>, value: BigInt) {
    let value_bytes = value.to_bytes_be();

    der_encoding.extend_from_slice(&value_bytes.1);
}


fn encode_public_key_der(modulus: &BigInt, public_exponent: &BigInt) -> Vec<u8> {
    let mut der_encoding = vec![0x30, 0x82, 0x01, 0xa, 0x02, 0x82, 0x01, 0x01, 0x00];

    encode_integer(&mut der_encoding, modulus.clone());
    der_encoding.push(0x02);
    der_encoding.push(0x03);
    encode_integer(&mut der_encoding, public_exponent.clone());

    der_encoding
}

fn der_to_pem(der_bytes: &[u8]) -> String {
    let base64_encoded = base64::encode(der_bytes);
    format!(
        "-----BEGIN RSA PUBLIC KEY-----\n{}\n-----END RSA PUBLIC KEY-----",
        base64_encoded
    )
}

fn base64_encode(bytes: &[u8]) -> String {
    let enc_lookup_table: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '+', '/'
    ];

    let mut result = String::new();
    let mut i = 0;
    let index_of_last_complete_triple = (bytes.len() / 3) * 3;

    // handle triples of input characters per loop
    while i < index_of_last_complete_triple {
        result.push(enc_lookup_table[(bytes[i] >> 2) as usize]);
        result.push(enc_lookup_table[((bytes[i] & 0x03) << 4 | (bytes[i + 1] & 0xF0) >> 4) as usize]);
        result.push(enc_lookup_table[((bytes[i + 1] & 0x0F) << 2 | (bytes[i + 2] & 0xC0) >> 6) as usize]);
        result.push(enc_lookup_table[(bytes[i + 2] & 0x3F) as usize]);
        i += 3;
    }

    if i < bytes.len() {
        let idx1 = bytes[i];
        let idx2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        result.push(enc_lookup_table[(idx1 >> 2) as usize]);
        result.push(enc_lookup_table[((idx1 & 0x03) << 4 | (idx2 & 0xF0) >> 4) as usize]);
        if i + 1 < bytes.len() {
            result.push(enc_lookup_table[((idx2 & 0x0F) << 2) as usize]);
        } else {
            result.push('=');
        }
        result.push('=');
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_generate_rsa_key() {
        let test = generate_rsa_key();
        println!("result: {}", test);
    }

    #[test]
    fn test_encode_public_key_der() {
        let modulus = BigInt::parse_bytes(b"EB506399F5C612F5A67A09C1192B92FAB53DB28520D859CE0EF6B7D83D40AA1C1DCE2C0720D15A0F531595CAD81BA5D129F91CC6769719F1435872C4BCD0521150A0263B470066489B918BFCA03CE8A0E9FC2C0314C4B096EA30717C03C28CA29E678E63D78ACA1E9A63BDB1261EE7A0B041AB53746D68B57B68BEF37B71382838C95DA8557841A3CA58109F0B4F77A5E929B1A25DC2D6814C55DC0F81CD2F4E5DB95EE70C706FC02C4FCA358EA9A82D8043A47611195580F89458E3DAB5592DEFE06CDE1E516A6C61ED78C13977AE9660A9192CA75CD72967FD3AFAFA1F1A2FF6325A5064D847028F1E6B2329E8572F36E708A549DDA355FC74A32FDD8DBA65", 16).unwrap();
        println!("test: {}", modulus);
        let public_exponent: BigInt = 65537.into();
        let der_encoding = encode_public_key_der(&modulus, &public_exponent);
        let mut i = 0; 
        for byte in &der_encoding {
            i += 1;
            print!("{:02x} ", byte);
        }
        println!("len: {}", i);
        let str = der_to_pem(&der_encoding);
        println!("{}", str);
        let str2 = base64_encode(&der_encoding);
        println!("{}", str2);
    }
}