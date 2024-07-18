use cp_library::extended_number::ExtNum;

#[test]
fn test_inf() {
    let a = ExtNum::<usize>::inf();
    assert!(a.is_inf());

    let a = ExtNum::<u128>::inf();
    assert!(a.is_inf());

    let a = ExtNum::<u64>::inf();
    assert!(a.is_inf());

    let a = ExtNum::<u32>::inf();
    assert!(a.is_inf());
}

#[test]
fn test_add() {
    let inf = ExtNum::<u32>::inf();

    // ふつうの足し算ができるパターン
    let a = ExtNum(100u32);
    let b = ExtNum(10u32);
    assert_eq!(a + b, ExtNum(110u32));

    // 右側が無限大のパターン
    let a = ExtNum::<u32>::inf();
    let b = ExtNum(10u32);
    assert_eq!(a + b, inf);

    // 左側が無限大のパターン
    assert_eq!(b + a, inf);

    // 両側が無限大のパターン
    assert_eq!(a + a, inf);

    // add assign で足される方が無限大のパターン
    let mut a = ExtNum::<u32>::inf();
    let b = ExtNum(10u32);
    a += b;
    assert_eq!(a, inf);

    // add assign で足す方が無限大のパターン
    let a = ExtNum::<u32>::inf();
    let mut b = ExtNum(10u32);
    b += a;
    assert_eq!(b, inf);
}
