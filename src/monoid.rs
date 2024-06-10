pub trait Monoid {
    type S; // 集合
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S; // 二項演算
    const E: Self::S; // 単位元
}

pub struct Add<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Max<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Min<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Update<T> {
    _marker: std::marker::PhantomData<T>,
}

macro_rules! impl_primitives {
    ($($t: ty), *) => {
        $(
            impl Monoid for Add<$t> {
                type S = $t;
                const E: Self::S = 0;
                fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
                    lhs + rhs
                }
            }

            impl Monoid for Max<$t> {
                type S = $t;
                const E: Self::S = <$t>::MIN;
                fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
                    std::cmp::max(*lhs, *rhs)
                }
            }

            impl Monoid for Min<$t> {
                type S = $t;
                const E: Self::S = <$t>::MAX;
                fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
                    std::cmp::min(*lhs, *rhs)
                }
            }

            impl Monoid for Update<$t> {
                type S = $t;
                const E: Self::S = <$t>::MAX;
                fn op(_: &Self::S, rhs: &Self::S) -> Self::S {
                    *rhs
                }
            }
        )*
    };
}

impl_primitives!(usize, isize, u128, i128, u64, i64);
