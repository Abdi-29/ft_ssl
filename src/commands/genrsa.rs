use crate::algorithms::primality::{lcm, mod_inverse, generate_prime};
use base64;
use num::{BigInt, One};

#[derive(Clone, Debug)]
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

pub fn encode_integer_2(value: &BigInt) -> Vec<u8> {
    let mut bytes = value.to_bytes_le();
    let len = bytes.1.len();
    if len > 1 && bytes.1[len - 1] == 0 {
        bytes.1.pop();
    }
    bytes.1
}

fn encode_sequence(sequences: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes:Vec<u8> = vec![];

    for seq in sequences {
        bytes.extend(seq);
    }

    let mut result = vec![0x30,0x82,0x0,0x40,0x2,0x1,0x0]; // Sequence type
    // result.extend(encode_length(bytes.len()));
    result.extend(bytes);
    result
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

pub fn encode_private_key(rsa: RsaKey) -> String {
    // let version = vec![0x02, 0x01, 0x00]; // Version
    let modulus = encode_integer_2(&rsa.modulus);
    let public_exponent = encode_integer_2(&rsa.public_exponent);
    let private_exponent = encode_integer_2(&rsa.private_exponent);
    let prime1 = encode_integer_2(&rsa.prime[0]);
    let prime2 = encode_integer_2(&rsa.prime[1]);
    let exponent1 = encode_integer_2(&rsa.exponent[0]);
    let exponent2 = encode_integer_2(&rsa.exponent[1]);
    let coefficient = encode_integer_2(&rsa.coefficient);

    let sequences = [
        // version,
        modulus,
        public_exponent,
        private_exponent,
        prime1,
        prime2,
        exponent1,
        exponent2,
        coefficient,
    ];
    println!("testing");
    let der_encoding = encode_sequence(&sequences);
    for byte in &der_encoding {
        print!("{:02x} ", byte);
    }
        
    let base64_encoded = base64::encode(&der_encoding);

    format!(
        "-----BEGIN RSA PRIVATE KEY-----\n{}\n-----END RSA PRIVATE KEY-----",
        base64_encoded
    )
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
        // let modulus = BigInt::parse_bytes(b"B5FE740396423479", 16).unwrap();
        let t = generate_rsa_key();
        println!("t: {:?}", t);
        // let modulus = t.modulus.clone();
        // println!("test: {}", modulus);
        // let public_exponent: BigInt = 65537.into();
        // // let public_exponent = t.public_exponent.clone();
        // let der_encoding = encode_public_key_der(&modulus, &public_exponent);
        // let mut i = 0;
        // for byte in &der_encoding {
        //     i += 1;
        //     print!("{:02x} ", byte);
        // }
        // println!("len: {}", i);
        // let str = der_to_pem(&der_encoding);
        // println!("{}", str);
        // let str2 = base64_encode(&der_encoding);
        // println!("{}", str2);
    }

    #[test]
    fn test_encode_private_key() {
        let public_exponent = BigInt::from(65537u64);
        let private_exponent = BigInt::from(8484707624939179073u64);
        let prime1 = BigInt::from(3754937401u64);
        let prime2 = BigInt::from(3672915913u64);
        let modulus = prime1.clone() * prime2.clone();
        let exponent1 = &private_exponent % (&prime1 - BigInt::one());
        let exponent2 = &private_exponent % (&prime2 - BigInt::one());
        let coefficient = mod_inverse(prime1.clone(), prime2.clone());

        let rsa_key = RsaKey {
            modulus: modulus.clone(),
            public_exponent: public_exponent.clone(),
            private_exponent: private_exponent.clone(),
            prime: [prime1, prime2],
            exponent: [exponent1, exponent2],
            coefficient,
        };

        let pem = encode_private_key(rsa_key.clone());
        println!("{}", pem);
        println!("prime1: {}\n prime2: {}\nmodulus: {}\nprivate: {}\nexpo1: {}\nexp2: {}\ncoef: {}", rsa_key.prime[0], rsa_key.prime[1], rsa_key.modulus, rsa_key.private_exponent, rsa_key.exponent[0], rsa_key.exponent[1], rsa_key.coefficient);
    }
}
