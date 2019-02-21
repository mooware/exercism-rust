use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_100() {
    assert_eq!(np::nth(100), 547);
}

#[test]
fn test_1000() {
    assert_eq!(np::nth(1000), 7927);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10000), 104743);
}
