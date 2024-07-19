use cp_library::extended_number::ExtendedNumber;

#[test]
fn test_inf() {
    let a = ExtendedNumber::<usize>::INF;
    assert!(a.is_inf());

    let a = ExtendedNumber::<u128>::INF;
    assert!(a.is_inf());

    let a = ExtendedNumber::<u64>::INF;
    assert!(a.is_inf());

    let a = ExtendedNumber::<u32>::INF;
    assert!(a.is_inf());
}

#[test]
fn test_add() {
    let inf = ExtendedNumber::<u32>::INF;

    // ふつうの足し算ができるパターン   w
    let a = ExtendedNumber(100u32);
    let b = ExtendedNumber(10u32);
    assert_eq!(a + b, ExtendedNumber(110u32));

    // 右側が無限大のパターン
    let a = ExtendedNumber::<u32>::INF;
    let b = ExtendedNumber(10u32);
    assert_eq!(a + b, inf);

    // 左側が無限大のパターン
    assert_eq!(b + a, inf);

    // 両側が無限大のパターン
    assert_eq!(a + a, inf);

    // add assign で足される方が無限大のパターン
    let mut a = ExtendedNumber::<u32>::INF;
    let b = ExtendedNumber(10u32);
    a += b;
    assert_eq!(a, inf);

    // add assign で足す方が無限大のパターン
    let a = ExtendedNumber::<u32>::INF;
    let mut b = ExtendedNumber(10u32);
    b += a;
    assert_eq!(b, inf);
}
