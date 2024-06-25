use crate::monoid::{Add, Max, Min, Monoid, Update};

pub trait ActMonoid {
    type A: Monoid;
    type X: Monoid;

    fn act(x: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S;
}

pub struct UpdateMin<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct UpdateMax<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct AddMin<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct AddMax<T> {
    _marker: std::marker::PhantomData<T>,
}

/// 区間加算, 区間和クエリに応える
/// <注意>
/// 初期化するとき、最初は長さが 1 の要素が挿入されるので
/// S::from(value)をすべての i に対して insert する必要がある
pub struct AddSum<T> {
    _marker: std::marker::PhantomData<T>,
}

#[derive(Clone, Copy)]
pub struct S<T> {
    value: T,
    length: usize,
}

impl<T: Copy> S<T> {
    pub fn from(value: T) -> Self {
        Self { value, length: 1 }
    }

    pub fn value(&self) -> T {
        self.value
    }
}

macro_rules! impl_primitives {
    ($($t: ty), *) => {
        $(
            impl ActMonoid for UpdateMin<$t> {
                type A = Update<$t>;
                type X = Min<$t>;

                fn act(_: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S {
                    *a
                }
            }

            impl ActMonoid for UpdateMax<$t> {
                type A = Update<$t>;
                type X = Max<$t>;

                fn act(_: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S {
                    *a
                }
            }

            impl ActMonoid for AddMin<$t> {
                type A = Add<$t>;
                type X = Min<$t>;

                fn act(x: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S {
                    x + a
                }
            }

            impl ActMonoid for AddMax<$t> {
                type A = Add<$t>;
                type X = Max<$t>;

                fn act(x: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S {
                    x + a
                }
            }

            impl Monoid for Add<S<$t>> {
                type S = S<$t>;
                const E: Self::S = S {
                    value: 0,
                    length: 0,
                };
                fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
                    S {
                        value: lhs.value + rhs.value,
                        length: lhs.length + rhs.length,
                    }
                }
            }

            impl ActMonoid for AddSum<$t> {
                type A = Add<$t>;
                type X = Add<S<$t>>;

                fn act(x: &<Self::X as Monoid>::S, a: &<Self::A as Monoid>::S) -> <Self::X as Monoid>::S {
                    S {
                        value: x.value + a * x.length as $t,
                        length: x.length,
                    }
                }
            }

            impl std::fmt::Display for S<$t> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.value)
                }
            }
        )*
    };
}

impl_primitives!(usize, isize, u128, i128, u64, i64);

pub struct LazySegTree<A: ActMonoid> {
    size: usize,
    lazy: Vec<<A::A as Monoid>::S>,
    tree: Vec<<A::X as Monoid>::S>,
}

impl<A: ActMonoid<A = F, X = X>, F: Monoid, X: Monoid> LazySegTree<A>
where
    <F as Monoid>::S: PartialEq + Copy,
    <X as Monoid>::S: Copy,
{
    /// self = [X::E; size]
    pub fn new(size: usize) -> Self
    where
        <F as Monoid>::S: Clone,
        <X as Monoid>::S: Clone,
    {
        Self {
            size,
            lazy: vec![<F as Monoid>::E; size << 1],
            tree: vec![<X as Monoid>::E; size << 1],
        }
    }

    /// self[i] <- x
    pub fn insert(&mut self, mut i: usize, x: &X::S) {
        i += self.size;

        for h in (1..=self._height()).rev() {
            self._propagate(i >> h);
        }

        self.tree[i] = *x;

        for h in 1..=self._height() {
            self._update(i >> h);
        }
    }

    /// return self[i]
    pub fn get(&mut self, mut i: usize) -> X::S {
        i += self.size;

        for h in (1..=self._height()).rev() {
            self._propagate(i >> h);
        }

        self.tree[i]
    }

    /// return Π_{i ∈ range} self[i]
    pub fn prod<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> X::S {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.size,
        };

        return self._prod(left, right);
    }

    /// (not recommended for use)
    pub fn display(&mut self) -> String
    where
        <X as Monoid>::S: std::fmt::Display,
    {
        let vs = (0..self.size).map(|i| self.get(i)).collect::<Vec<_>>();
        format!(
            "{}",
            vs.iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn _prod(&mut self, mut l: usize, mut r: usize) -> X::S {
        if l >= r {
            return X::E;
        }

        l += self.size;
        r += self.size;

        for h in (1..=self._height()).rev() {
            if ((l >> h) << h) != l {
                self._propagate(l >> h);
            }

            if ((r >> h) << h) != r {
                self._propagate((r - 1) >> h);
            }
        }

        let mut vl = X::E;
        let mut vr = X::E;

        while l < r {
            if l & 1 == 1 {
                vl = X::op(&vl, &self.tree[l]);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                vr = X::op(&self.tree[r], &vr);
            }

            l >>= 1;
            r >>= 1;
        }

        X::op(&vl, &vr)
    }

    /// for each i in range, act f to self[i]
    pub fn apply<R: std::ops::RangeBounds<usize>>(&mut self, range: R, f: &F::S) {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.size,
        };

        return self._act(left, right, f);
    }

    fn _act(&mut self, mut l: usize, mut r: usize, f: &F::S) {
        if l >= r {
            return;
        }

        l += self.size;
        r += self.size;

        for h in (1..=self._height()).rev() {
            if ((l >> h) << h) != l {
                self._propagate(l >> h);
            }

            if ((r >> h) << h) != r {
                self._propagate((r - 1) >> h);
            }
        }

        let (_l, _r) = (l, r);

        while l < r {
            if l & 1 == 1 {
                self._apply(l, f);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                self._apply(r, f);
            }

            l >>= 1;
            r >>= 1;
        }

        l = _l;
        r = _r;

        for h in 1..=self._height() {
            if ((l >> h) << h) != l {
                self._update(l >> h);
            }

            if ((r >> h) << h) != r {
                self._update((r - 1) >> h);
            }
        }
    }

    #[inline(always)]
    fn _update(&mut self, i: usize) {
        self.tree[i] = X::op(&self.tree[i << 1], &self.tree[i << 1 | 1]);
    }

    #[inline(always)]
    fn _apply(&mut self, i: usize, f: &F::S) {
        self.tree[i] = A::act(&self.tree[i], f);

        if i < self.size {
            self.lazy[i] = F::op(&self.lazy[i], f);
        }
    }

    #[inline(always)]
    fn _propagate(&mut self, i: usize) {
        if self.lazy[i] != F::E {
            let f = self.lazy[i];
            self._apply(i << 1, &f);
            self._apply(i << 1 | 1, &f);
            self.lazy[i] = F::E;
        }
    }

    #[inline(always)]
    fn _height(&self) -> u32 {
        self.size.next_power_of_two().trailing_zeros()
    }
}
