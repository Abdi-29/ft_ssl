use crate::algorithms::primality::is_prime; 

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(7, 30), true);
}