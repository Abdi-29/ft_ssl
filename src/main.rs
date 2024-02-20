pub mod algorithms;
pub mod commands;

use crate::algorithms::primality::generate_prime;

use crate::algorithms::primality::is_prime;
use crate::commands::genrsa::generate_rsa_key;

fn main() {
    println!("Hello, world!");
    let n = 7;
    let k = 95;
    println!("testing prime: {}", is_prime(n, k));
    println!("testing generate_prime {}", generate_prime());
    let key = generate_rsa_key();
    println!("module {}", key.modulus);
    println!("public {}", key.public_exponent);
    println!("private {}", key.private_exponent);
}
