use crate::monoid::Monoid;

pub struct DualSegTree<M: Monoid> {
    size: usize,
    tree: Vec<M::S>,
}

impl<M: Monoid<S = S>, S: Copy + PartialEq> DualSegTree<M> {
    /// self = [M::E; size]
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            tree: vec![M::E; size << 1],
        }
    }

    /// return self[i]
    pub fn get(&mut self, mut i: usize) -> M::S {
        i += self.size;
        self.propagate_above(i);
        self.tree[i]
    }

    /// act x to range
    pub fn act<R: std::ops::RangeBounds<usize>>(&mut self, range: R, x: &M::S) {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.size,
        };

        self._act(l, r, x)
    }

    /// act x to l..r  
    fn _act(&mut self, mut l: usize, mut r: usize, x: &M::S) {
        l += self.size;
        r += self.size;

        self.propagate_above(l);
        self.propagate_above(r - 1);

        while l < r {
            if l & 1 == 1 {
                self.tree[l] = M::op(&self.tree[l], x);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                self.tree[r] = M::op(&self.tree[r], x);
            }

            l >>= 1;
            r >>= 1;
        }
    }

    #[inline(always)]
    fn propagate(&mut self, i: usize) {
        if self.tree[i] != M::E {
            self.tree[i << 1] = M::op(&self.tree[i << 1], &self.tree[i]);
            self.tree[i << 1 | 1] = M::op(&self.tree[i << 1 | 1], &self.tree[i]);
            self.tree[i] = M::E;
        }
    }

    #[inline(always)]
    fn propagate_above(&mut self, i: usize) {
        let h = self.size.next_power_of_two().trailing_zeros();
        for h in (1..=h).rev() {
            self.propagate(i >> h);
        }
    }
}
