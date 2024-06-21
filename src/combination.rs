use crate::modint::ModInt;

pub struct CombinationTable<const P: u32> {
    size: usize,
    f: Vec<ModInt<P>>,
    fi: Vec<ModInt<P>>,
    inv: Vec<ModInt<P>>,
}

impl<const P: u32> CombinationTable<P> {
    pub fn new() -> Self {
        Self {
            size: 1,
            f: vec![ModInt::<P>::from_raw(1), ModInt::<P>::from_raw(1)],
            fi: vec![ModInt::<P>::from_raw(1), ModInt::<P>::from_raw(1)],
            inv: vec![ModInt::<P>::from_raw(0), ModInt::<P>::from_raw(1)],
        }
    }

    fn resize(&mut self, new_length: usize) {
        while self.size < new_length {
            self.size += 1;
            let f = self.f[self.size - 1] * self.size;
            let inv = -self.inv[P as usize % self.size] * (P as usize / self.size);
            let fi = self.fi[self.size - 1] * inv;

            self.f.push(f);
            self.fi.push(fi);
            self.inv.push(inv);
        }
    }

    /// return \binom{n}{k}
    pub fn combination(&mut self, n: usize, k: usize) -> ModInt<P> {
        self.resize(n);
        if n < k {
            return ModInt::<P>::new();
        } else {
            return self.f[n] * self.fi[k] * self.fi[n - k];
        }
    }

    /// return n!
    pub fn factorial(&mut self, n: usize) -> ModInt<P> {
        self.resize(n);
        self.f[n]
    }
}
