pub trait ExtendedNumber {
    type Item;
    fn is_inf(&self) -> bool;
    fn inf() -> Self;
    fn zero() -> Self;
    fn from(vale: Self::Item) -> Self;
}

/// the type of integer which regard `T::MAX` as infinity
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ExtNum<T>(pub T);

macro_rules! impl_primitives {
    ($($t: ty), *) => {
        $(
            impl ExtNum<$t> {
                pub fn is_inf(&self) -> bool {
                    self.0 == <$t>::MAX
                }

                pub fn inf() -> Self {
                    Self(<$t>::MAX)
                }
            }

            impl std::ops::Add for ExtNum<$t> {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    if self.is_inf() || rhs.is_inf() {
                        return Self(<$t>::MAX)
                    } else {
                        return Self(self.0 + rhs.0)
                    }
                }
            }

            impl std::ops::Sub for ExtNum<$t> {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    assert!(!rhs.is_inf());

                    if self.is_inf() {
                        return Self(<$t>::MAX)
                    } else {
                        return Self(self.0 - rhs.0)
                    }
                }
            }

            impl std::ops::AddAssign for ExtNum<$t> {
                fn add_assign(&mut self, rhs: Self) {
                    *self = *self + rhs
                }
            }

            impl std::ops::SubAssign for ExtNum<$t> {
                fn sub_assign(&mut self, rhs: Self) {
                    *self = *self - rhs
                }
            }

            impl std::fmt::Display for ExtNum<$t> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        if self.is_inf() {
                            "INF".to_string()
                        } else {
                            format!("{}", self.0)
                        }
                    )
                }
            }

            impl ExtendedNumber for ExtNum<$t> {
                type Item = $t;
                fn is_inf(&self) -> bool {
                    self.is_inf()
                }
                fn inf() -> Self {
                    Self::inf()
                }
                fn zero() -> Self {
                    Self(0)
                }
                fn from(value: $t) -> Self {
                    Self(value)
                }
            }
        )*
    };
}

impl_primitives!(usize, u128, u64, u32);
