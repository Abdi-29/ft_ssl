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
