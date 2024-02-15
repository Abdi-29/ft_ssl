pub mod algorithms;

use ft_ssl::algorithms::primality::generate_prime;

use crate::algorithms::primality::is_prime;

fn main() {
    println!("Hello, world!");
    let n = 7;
    let k = 95;
    println!("testing prime: {}", is_prime(n, k));
    println!("testing generate_prime {}", generate_prime());
}
