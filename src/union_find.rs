pub struct UnionFind {
    data: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        return Self {
            data: vec![-1; size],
        };
    }

    fn _find(&mut self, v: usize) -> usize {
        assert!(v < self.data.len());
        if self.data[v] < 0 {
            return v;
        }

        self.data[v] = self._find(self.data[v] as usize) as i32;
        return self.data[v] as usize;
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        assert!(v < self.data.len() && u < self.data.len());
        self._find(u) == self._find(v)
    }

    pub fn unite(&mut self, mut a: usize, mut b: usize) -> () {
        assert!(a < self.data.len() && b < self.data.len());
        a = self._find(a);
        b = self._find(b);

        if a == b {
            return;
        }
        if self.data[a] > self.data[b] {
            (a, b) = (b, a);
        }

        self.data[a] += self.data[b];
        self.data[b] = a as i32;
    }

    pub fn size(&mut self, mut v: usize) -> i32 {
        assert!(v < self.data.len());
        v = self._find(v);
        -self.data[v]
    }
}
