use cp_library::combination::CombinationTable;
use cp_library::modint::ModInt;

type MInt = ModInt<998244353>;

macro_rules! mint {
    ($k: expr) => {
        MInt::from($k)
    };
}

#[test]
fn test_factorial() {
    let mut ct: CombinationTable<998244353> = CombinationTable::new();

    assert_eq!(ct.factorial(3), mint!(6));
    assert_eq!(ct.factorial(5), mint!(120));
    assert_eq!(ct.factorial(1000), mint!(421678599));
    assert_eq!(ct.factorial(10000), mint!(777990065));
    assert_eq!(ct.factorial(100000), mint!(215582594));
}

#[test]
fn test_combination() {
    let mut ct: CombinationTable<998244353> = CombinationTable::new();

    assert_eq!(ct.combination(5, 3), mint!(5 * 4 / 2));
    assert_eq!(ct.combination(1000, 999), mint!(1000));
    assert_eq!(ct.combination(10, 1), ct.combination(10, 9));
    assert_eq!(
        ct.combination(100_000, 12345),
        ct.combination(100_000, 100_000 - 12345)
    );
}
