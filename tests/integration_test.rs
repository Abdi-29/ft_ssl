use ft_ssl::algorithms::primality::is_prime;

#[test]
fn test_is_prime() {
    let test_cases = [
        (2, true),
        (3, true),
        (5, true),
        (17, true),
        (61, true),
        (73, true),
        (3197041061052982553, true),
        (77, false),
        (14, false),
        (1, false),
        (57, false),
        (72, false),
        (88, false),
        (28, false),
    ];

    for (number, expected) in test_cases.iter() {
        assert_eq!(is_prime(*number, 5), *expected);
    }

}

#[test]
#[should_panic(expected = "Probability value must be between 0 to 100, got")]
fn test_is_prime_panic() {
    is_prime(2, 101);
}