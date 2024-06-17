use cp_library::modint::ModInt;

type MInt998 = ModInt<998244353>;

macro_rules! mint {
    ($k: expr) => {
        MInt998::from($k)
    };
}

#[test]
fn test_from() {
    let a = mint!(-10isize);
    assert_eq!(a, mint!(998244353 - 10));

    let a = mint!(-998244353);
    assert_eq!(a, mint!(0));

    let a = mint!(998244354);
    assert_eq!(a, MInt998::from_raw(1));

    let a = mint!(998244353usize << 30);
    assert_eq!(a, mint!(0));
}

#[test]
fn test_add() {
    assert_eq!(mint!(998244352) + mint!(1), mint!(0));
    assert_eq!(mint!(998244352) + mint!(998244352), mint!(-2));
}

#[test]
fn test_addassign() {
    let mut a = mint!(998244352);
    a += 2;
    assert_eq!(a, mint!(1));
}

#[test]
fn test_sub() {
    assert_eq!(mint!(0) - mint!(1), mint!(998244352));
    assert_eq!(mint!(0) - mint!(998244352), mint!(1));
}

#[test]
fn test_subassign() {
    let mut a = mint!(0);
    a -= 1;
    assert_eq!(a, mint!(998244352));
}

#[test]
fn test_mul() {
    assert_eq!(mint!(998244352) * mint!(998244352), mint!(1));
    assert_eq!(mint!(0) * mint!(1), mint!(0));
    assert_eq!(
        mint!(1000000) * mint!(1000000),
        mint!(1000000000000u64 % 998244353)
    );
}

#[test]
fn test_mulassign() {
    let mut a = mint!(998244352);
    a *= 1234567;
    assert_eq!(a, mint!(-1234567));
}

#[test]
fn test_div() {
    assert_eq!(mint!(1) / mint!(2), mint!(499122177));
    assert_eq!(mint!(0) / mint!(1), mint!(0));
    assert_eq!(mint!(998244352) / mint!(998244352), mint!(1));
}

#[test]
fn test_divassign() {
    let mut a = mint!(1);
    a /= 2;
    assert_eq!(a, mint!(1) / mint!(2));
}

#[test]
fn test_pow() {
    assert_eq!(mint!(2).pow(10), mint!(1024));
    assert_eq!(mint!(10).pow(10), mint!(10_000_000_000usize % 998244353));
    assert_eq!(mint!(998244352).pow(2), mint!(1));
}
