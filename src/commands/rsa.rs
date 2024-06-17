use num::BigInt;
use super::genrsa::RsaKey;
use std::{fs::File, io::Read};

pub struct RsaOption {
    inform: String,
    outform: String,
    infile: Option<String>,
    passing: Option<String>,
    outfile: Option<String>,
    passout: Option<String>,
    des: bool,
    text: bool,
    nout: bool,
    modulus: bool,
    check: bool,
    pubin: bool,
    pubout: bool
}

impl Default for RsaOption {
    fn default() -> Self {
        RsaOption {
            inform: "PEM".to_string(),
            outform: "PEM".to_string(),
            infile: None,
            passing: None,
            outfile: None,
            passout: None,
            des: false,
            text: false,
            nout: false,
            modulus: false,
            check: false,
            pubin: false,
            pubout: false
        }
    }
}

pub fn rsa_command(args: &[String]) -> RsaOption {
    let mut option = RsaOption::default();
    let mut i = 0;
    let len = args.len();

    while i < len {
        match args[i].as_str() {
            "-inform" => {
                if i + 1 < len {
                    if args[i + 1] != "PEM" {
                        eprintln!("Only PEM is allowed for -inform");
                        std::process::exit(1);
                    }
                    option.inform = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Missing value for -inform");
                    std::process::exit(1);
                }
            },
            "-outform" => {
                if i + 1 < len {
                    if args[i + 1] != "PEM" {
                        eprintln!("Only PEM is allowed for -outform");
                        std::process::exit(1);
                    }
                    option.outform = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Missing value for -outform");
                    std::process::exit(1);
                }
            },
            "-in" => {
                if i + 1 < len {
                    option.infile = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -in");
                    std::process::exit(1);
                }
            },
            "-passin" => {
                if i + 1 < len {
                    option.passing = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -passing");
                    std::process::exit(1);
                }
            },
            "-out" => {
                if i + 1 < len {
                    option.outfile = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -out");
                    std::process::exit(1);
                }
            },
            "-passout" => {
                if i + 1 < len {
                    option.passout = Some(args[i + 1].clone());
                } else {
                    eprintln!("Missing value for -passout");
                    std::process::exit(1);
                }
            },
            "-des" => {
                if i + 1 < len {
                    option.des = true;
                }
            },
            "-text" => {
                if i + 1 < len {
                option.text = true;
            }
            },
            "-noout" => {
                if i + 1 < len {
                    option.nout = true;
                }
            },
            "-modulus" => {
                if i + 1 < len {
                option.modulus = true;
            }
            },
            "-check" => {
                if i + 1 < len {
                option.check = true;
            }
            },
            "-pubin" => {
                if i + 1 < len {
                option.pubin = true;
            }
            },
            "-pubout" => {
                if i + 1 < len {
                option.pubout = true;
            }
            },
            _=> {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    option
}

pub fn parse_private_key(option: &RsaOption) -> RsaKey {
    let pem_key = read_key(option.infile.as_ref());
    let pem_key = remove_pem_header(&pem_key);
    let decoded_key = base64_decode(&pem_key);
    parse_asn1_der(&decoded_key)
}

fn read_key(infile: Option<&String>) -> String {
    let mut content = String::new();
    if let Some(filename) = infile {
        let mut file = File::open(filename).expect("unable to open file");
        file.read_to_string(&mut content).expect("unable to read file");
    }
    content
}

fn remove_pem_header(pem: &String) -> String {
    pem.replace("-----BEGIN RSA PRIVATE KEY-----", "")
        .replace("-----END RSA PRIVATE KEY-----", "")
        .replace("\n", "")
}

fn base64_decode(input: &str) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut temp = 0;
    let mut bits = 0;

    for byte in input.bytes() {
        if byte == b'=' {
            break;
        }
        let val = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'-' => 63,
            _ => continue,
        };
        temp = (temp << 6) | val as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            buffer.push((temp >> bits) as u8);
        }
    }
    buffer
}

fn parse_asn1_der(decoded_key: &[u8]) -> RsaKey {
    let mut bytes = decoded_key;

    assert!(bytes[0] == 0x30);
    let sequence_len = (bytes[1] as usize) << 8 | (bytes[2] as usize);
    bytes = &bytes[3..];

    let version = decode_integer(&mut bytes);
    let modulus = decode_integer(&mut bytes);
    let public_exponent = decode_integer(&mut bytes);
    let private_exponent = decode_integer(&mut bytes);
    let prime1 = decode_integer(&mut bytes);
    let prime2 = decode_integer(&mut bytes);
    let exponent1 = decode_integer(&mut bytes);
    let exponent2 = decode_integer(&mut bytes);
    let coefficient = decode_integer(&mut bytes);

    RsaKey {
        modulus,
        public_exponent,
        private_exponent,
        prime: [prime1, prime2],
        exponent: [exponent1, exponent2],
        coefficient
    }
}

fn decode_integer(bytes: &mut &[u8]) -> BigInt {
    assert!(bytes[0] == 0x02);
    let len = bytes[1] as usize;
    let bigint_byte = &bytes[2..2 + len];
    *bytes = &bytes[2 + len..];
    BigInt::from_signed_bytes_be(bigint_byte)
}
