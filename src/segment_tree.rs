use crate::monoid::Monoid;

pub struct SegmentTree<M: Monoid> {
    size: usize,
    tree: Vec<M::S>,
}

impl<M: Monoid<S = S>, S: Clone + Copy> SegmentTree<M> {
    /// self = [e; size]
    pub fn new(size: usize) -> Self {
        return Self {
            size: size,
            tree: vec![M::E; 2 * size],
        };
    }

    /// self = array
    pub fn from(array: &[M::S]) -> Self {
        let size = array.len();
        let mut tree = vec![M::E; 2 * size];

        for i in 0..size {
            tree[i + size] = array[i];
        }

        for i in (1..size).rev() {
            tree[i] = M::op(&tree[i << 1], &tree[i << 1 | 1]);
        }

        return Self { size, tree };
    }

    /// self[i] <- s
    pub fn update(&mut self, mut i: usize, s: S) {
        i += self.size;

        self.tree[i] = s;

        while i > 1 {
            i >>= 1;
            self.tree[i] = M::op(&self.tree[i << 1], &self.tree[i << 1 | 1]);
        }
    }

    /// get self[i]
    pub fn get(&self, i: usize) -> S {
        return self.tree[i + self.size];
    }

    /// calculate Π_{i ∈ range} self[i]
    pub fn prod<R: std::ops::RangeBounds<usize>>(&self, range: R) -> S {
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

    fn _prod(&self, mut left: usize, mut right: usize) -> S {
        left += self.size;
        right += self.size;
        let (mut sl, mut sr) = (M::E, M::E);

        while left < right {
            if left & 1 == 1 {
                sl = M::op(&sl, &self.tree[left]);
                left += 1;
            }

            if right & 1 == 1 {
                right -= 1;
                sr = M::op(&self.tree[right], &sr);
            }

            left >>= 1;
            right >>= 1;
        }

        return M::op(&sl, &sr);
    }
}
