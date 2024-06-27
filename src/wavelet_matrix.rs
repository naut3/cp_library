pub struct WaveletMatrix {
    bvs: Vec<BitVector>,
    length: usize,
    height: usize,
}

impl WaveletMatrix {
    pub fn from(array: &[u64], height: usize) -> Self {
        let mut bvs = vec![];
        let mut array = array.to_vec();

        for i in (0..height).rev() {
            // i bit 目で安定ソートする
            let mut a0 = vec![];
            let mut a1 = vec![];

            let mut bv = vec![];

            for (j, &a) in array.iter().enumerate() {
                if j % 64 == 0 {
                    bv.push(0);
                }

                bv[j / 64] |= ((a >> i) & 1) << (j % 64);

                if (a >> i) & 1 == 1 {
                    a1.push(a);
                } else {
                    a0.push(a);
                }
            }

            bvs.push(BitVector::from(&bv));
            a0.append(&mut a1);
            array = a0;
        }

        bvs.reverse();

        Self {
            bvs,
            length: array.len(),
            height,
        }
    }

    /// i 番目の要素の値を取得する
    pub fn access(&self, mut i: usize) -> u64 {
        // 上のbitから順番に位置を変更しながら走査すればよい
        let mut ret = 0u64;

        for j in (0..self.height).rev() {
            if self.bvs[j].access(i) {
                ret |= 1 << j;

                // 0の数 + 1の数
                i = self.bvs[j].rank(i, true) as usize
                    + self.bvs[j].rank(self.length - 1, false) as usize
                    - 1;
            } else {
                i = self.bvs[j].rank(i, false) as usize - 1;
            }
        }

        return ret;
    }

    /// [l, r) の中で k 番目に小さい値を求める (0 <= k)
    pub fn quantile(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
        let mut ret = 0u64;

        for j in (0..self.height).rev() {
            let l0 = if l > 0 {
                self.bvs[j].rank(l - 1, false)
            } else {
                0
            };
            let r0 = if r > 0 {
                self.bvs[j].rank(r - 1, false)
            } else {
                0
            };

            if k as u32 + l0 < r0 {
                l = l0 as usize;
                r = r0 as usize;
            } else {
                ret |= 1 << j;
                k -= r0 as usize - l0 as usize;
                let count_zeros = self.bvs[j].rank(self.length - 1, false);
                l += (count_zeros - l0) as usize;
                r += (count_zeros - r0) as usize;
            }
        }

        return ret;
    }
}

impl std::fmt::Display for WaveletMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..64 {
            let _ = writeln!(f, "{}", self.bvs[i]);
        }
        Ok(())
    }
}

pub struct BitVector {
    row: Vec<u64>,
    cs: Vec<u32>,
}

impl BitVector {
    pub fn from(array: &[u64]) -> Self {
        let mut cs = vec![0];

        for (i, a) in array.iter().enumerate() {
            cs.push(cs[i] + a.count_ones());
        }

        return Self {
            row: array.to_vec(),
            cs,
        };
    }

    /// i bit 目の値を返す (i は 0 以上)
    pub fn access(&self, i: usize) -> bool {
        (self.row[i / 64] >> (i % 64)) & 1 == 1
    }

    /// i bit 目までの b の数を数える (0 bit 目の存在に注意)
    pub fn rank(&self, i: usize, b: bool) -> u32 {
        if i % 64 == 63 {
            let c = self.cs[i / 64 + 1];

            if b {
                c
            } else {
                i as u32 + 1 - c
            }
        } else {
            let c = self.cs[i / 64];
            let dc = (self.row[i / 64] & ((1 << ((i % 64) + 1)) - 1)).count_ones();

            if b {
                c + dc
            } else {
                i as u32 + 1 - (c + dc)
            }
        }
    }

    /// とりあえず使わないので後回しにする
    #[allow(unused)]
    fn select(&self, i: usize, b: bool) -> usize {
        todo!()
    }
}

impl std::fmt::Display for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.row
                .iter()
                .map(|x| format!("{:>064b}", x).chars().rev().collect::<String>())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
