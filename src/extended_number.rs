pub trait HasMaxValue {
    type S;
    const M: Self::S;
}

macro_rules! impl_primitives {
    ($($t: ty), *) => {
        $(
            impl HasMaxValue for $t {
                type S = $t;
                const M: Self::S = <$t>::MAX;
            }
        )*
    };
}

impl_primitives!(usize, u128, u64, u32);

/// the type of integer which regard `T::MAX` as infinity
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Debug)]
pub struct ExtendedNumber<N>(pub N);

impl<N: HasMaxValue<S = N> + PartialEq + Eq> ExtendedNumber<N> {
    pub const INF: Self = Self(N::M);

    pub fn is_inf(&self) -> bool {
        self.0 == N::M
    }
}

impl<N: HasMaxValue<S = N> + PartialEq + Eq + Clone + std::ops::Add<Output = N>> std::ops::Add
    for ExtendedNumber<N>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_inf() || rhs.is_inf() {
            Self::INF
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

impl<N: HasMaxValue<S = N> + PartialEq + Eq + Copy + std::ops::Add<Output = N>> std::ops::AddAssign
    for ExtendedNumber<N>
{
    fn add_assign(&mut self, rhs: Self) {
        if self.is_inf() {
            return;
        }
        if rhs.is_inf() {
            *self = Self::INF;
            return;
        }
        *self = Self(self.0 + rhs.0)
    }
}

impl<N> From<N> for ExtendedNumber<N> {
    fn from(value: N) -> Self {
        Self(value)
    }
}

impl<N: HasMaxValue<S = N> + PartialEq + Eq + Clone + std::fmt::Display> std::fmt::Display
    for ExtendedNumber<N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self == &Self::INF {
                "INF".to_string()
            } else {
                format!("{}", self.0)
            }
        )
    }
}
