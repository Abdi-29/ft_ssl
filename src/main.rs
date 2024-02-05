pub mod algorithms;

use crate::algorithms::primality::is_prime;

fn main() {
    println!("Hello, world!");
    let n = 37;
    let k = 95;
    println!("testing prime: {}", is_prime(n, k));
}
