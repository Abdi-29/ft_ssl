use crate::algorithms::primality::{lcm, mod_inverse};
use crate::generate_prime;
use base64;
use num::bigint::ToBigInt;
use num::{BigInt, One};
use rand::seq;

pub struct RsaKey {
    pub modulus: BigInt,
    pub public_exponent: BigInt,
    pub private_exponent: BigInt,
    pub prime: [BigInt; 2],
    pub exponent: [BigInt; 2],
    pub coefficient: BigInt,
}

pub fn generate_rsa_key() -> RsaKey {
    let (p, q) = generate_prime();
    let modulus: u64 = p * q;
    println!("p: {}, q: {} and modulus: {}", p, q, modulus);

    let a: BigInt = p.try_into().unwrap();
    let b: BigInt = q.try_into().unwrap();
    let totient = lcm(&(a.clone() - BigInt::one()), &(b.clone() - 1));
    let modulus: BigInt = a.clone() * b.clone();
    let public_exponent: BigInt = 65537.into();
    let private_exponent: BigInt =
        mod_inverse(public_exponent.clone(), totient.clone().try_into().unwrap());
    let coefficient: BigInt = mod_inverse(b.clone(), a.clone());
    RsaKey {
        modulus,
        public_exponent: public_exponent.clone(),
        private_exponent: private_exponent.clone(),
        prime: [a.clone(), b.clone()],
        exponent: [
            private_exponent.clone() % (a.clone() - BigInt::one()),
            private_exponent.clone() % (b.clone() - BigInt::one()),
        ],
        coefficient,
    }
}

pub fn encode_integer_2(value: BigInt) -> Vec<u8> {
    let mut bytes = value.to_bytes_le();
    let len = bytes.1.len();
    if len > 1 && bytes.1[len - 1] == 0 {
        bytes.1.pop();
    }
    bytes.1
}

pub fn encode_sequence(sequences: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = vec![];

    for seq in sequences {
        bytes.extend(encode_length(seq.len()));
        bytes.extend(seq);
    }
    bytes
}

pub fn encode_length(length: usize) -> Vec<u8> {
    if length < 127 {
        vec![length as u8]
    } else {
        let mut bytes = vec![];
        let mut len = length;
        while len > 0 {
            bytes.push((len & 0xFF) as u8);
            len >>= 8;
        }
        bytes.push(0x80 | bytes.len() as u8);
        bytes.reverse();
        bytes
    }
}

pub fn encode_private_key(rsa: RsaKey) {
    
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
const ENC_LOOKUP_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn base64_encode(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    let index_of_last_complete_triple = (bytes.len() / 3) * 3;

    // handle triples of input characters per loop
    while i < index_of_last_complete_triple {
        result.push(ENC_LOOKUP_TABLE[(bytes[i] >> 2) as usize]);
        result
            .push(ENC_LOOKUP_TABLE[((bytes[i] & 0x03) << 4 | (bytes[i + 1] & 0xF0) >> 4) as usize]);
        result.push(
            ENC_LOOKUP_TABLE[((bytes[i + 1] & 0x0F) << 2 | (bytes[i + 2] & 0xC0) >> 6) as usize],
        );
        result.push(ENC_LOOKUP_TABLE[(bytes[i + 2] & 0x3F) as usize]);
        i += 3;
    }

    if i < bytes.len() {
        let idx1 = bytes[i];
        let idx2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        result.push(ENC_LOOKUP_TABLE[(idx1 >> 2) as usize]);
        result.push(ENC_LOOKUP_TABLE[((idx1 & 0x03) << 4 | (idx2 & 0xF0) >> 4) as usize]);
        if i + 1 < bytes.len() {
            result.push(ENC_LOOKUP_TABLE[((idx2 & 0x0F) << 2) as usize]);
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
        let module = test.modulus.to_bytes_be().1.len();
        let public = test.public_exponent.to_bytes_be().1.len();
        let private = test.private_exponent.to_bytes_be().1.len();
        let prime: usize = test.prime.iter().map(|p| p.to_bytes_be().1.len()).sum();
        let exponent: usize = test.exponent.iter().map(|e| e.to_bytes_be().1.len()).sum();
        let coe = test.coefficient.to_bytes_be().1.len();

        let tot = module + public + private + prime + exponent + coe + 8 as usize;
        let c = tot.to_be_bytes();
        let a = base64::encode(c);
        println!("testing: {:?}", a);
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

    #[test]
    fn test_encode_private_key() {
        let version = 0;
        let modulus: BigUint = 14012112600880994;

        let sequence = RsaKey {
            modulus,
            public_exponent: public_exponent.clone(),
            private_exponent: private_exponent.clone(),
            prime: [a.clone(), b.clone()],
            exponent: [
                private_exponent.clone() % (a.clone() - BigInt::one()),
                private_exponent.clone() % (b.clone() - BigInt::one()),
            ],
            coefficient,
        };
    }
}
