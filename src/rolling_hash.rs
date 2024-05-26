pub struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl RollingHash {
    const MOD: u64 = (1_u64 << 61) - 1;
    const MASK_30: u64 = (1_u64 << 30) - 1;
    const MASK_31: u64 = (1_u64 << 31) - 1;
    const MASK_61: u64 = (1_u64 << 61) - 1;
    const BASE: u64 = 100;
    const STR_BASE: char = 'a';

    fn mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & Self::MASK_31;
        let bu = b >> 31;
        let bd = b & Self::MASK_31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & Self::MASK_30;

        Self::cmod(au * bu * 2 + midu + (midd << 31) + ad * bd)
    }

    fn cmod(x: u64) -> u64 {
        let xu = x >> 61;
        let xd = x & Self::MASK_61;
        let ret = xu + xd;
        if ret >= Self::MOD {
            ret - Self::MOD
        } else {
            ret
        }
    }

    /// Prepare to calc hash value of given string s's substring
    pub fn from(s: &[char]) -> Self {
        let length = s.len();

        let mut hash = vec![0];
        let mut pow = vec![1];

        for i in 0..length {
            hash.push(
                Self::cmod(Self::mul(hash[i], Self::BASE)) + s[i] as u64 + 1
                    - Self::STR_BASE as u64,
            );
            pow.push(Self::cmod(Self::mul(pow[i], Self::BASE)));
        }

        Self {
            hash: hash,
            pow: pow,
        }
    }

    fn _h(&self, l: usize, r: usize) -> u64 {
        Self::cmod(self.hash[r] + Self::MOD * 4 - Self::mul(self.hash[l], self.pow[r - l]))
    }

    /// Calculate hash value of s[range]
    pub fn hash<R: std::ops::RangeBounds<usize>>(&self, range: R) -> u64 {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.hash.len() - 1,
        };

        self._h(left, right)
    }
}
