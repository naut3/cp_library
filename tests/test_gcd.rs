use cp_library::gcd::gcd;

#[test]
fn test_gcd_i32() {
    assert_eq!(gcd(15, 10), 5i32);
}

#[test]
fn test_gcd_u32() {
    assert_eq!(gcd(15, 10), 5u32);
}
