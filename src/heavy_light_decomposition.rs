use crate::graph::UndirectedGraph;

pub struct HeavyLightDecomposition<W> {
    graph: UndirectedGraph<W>,
    vid: Vec<usize>,
    head: Vec<usize>,
    size: Vec<u32>,
    parent: Vec<usize>,
}

impl<W: Copy> HeavyLightDecomposition<W> {
    /// 与えられたgraph, rootの組に対してHL分解を行う。
    pub fn new(graph: UndirectedGraph<W>, root: usize) -> Self {
        let n = graph.size;
        let mut ret = Self {
            graph,
            vid: vec![usize::MAX; n],
            head: vec![0; n],
            size: vec![1; n],
            parent: vec![0; n],
        };

        let mut k = 0;
        ret.dfs_1(root, usize::MAX);
        ret.head[root] = root;
        ret.dfs_2(root, usize::MAX, &mut k);

        return ret;
    }

    fn dfs_1(&mut self, v: usize, p: usize) {
        if self.graph.adjacency_list[v].len() != 0 && self.graph.adjacency_list[v][0].0 == p {
            let l = self.graph.adjacency_list[v].len();
            (
                self.graph.adjacency_list[v][0],
                self.graph.adjacency_list[v][l - 1],
            ) = (
                self.graph.adjacency_list[v][l - 1],
                self.graph.adjacency_list[v][0],
            );
        }

        for (i, &(nxt, _)) in self.graph.adjacency_list[v].clone().iter().enumerate() {
            if nxt == p {
                continue;
            }
            self.dfs_1(nxt, v);
            self.size[v] += self.size[nxt];

            let hv = self.graph.adjacency_list[v][0].0;
            if self.size[nxt] > self.size[hv] {
                (
                    self.graph.adjacency_list[v][0],
                    self.graph.adjacency_list[v][i],
                ) = (
                    self.graph.adjacency_list[v][i],
                    self.graph.adjacency_list[v][0],
                );
            }
        }
    }

    fn dfs_2(&mut self, v: usize, p: usize, k: &mut usize) {
        self.parent[v] = p;
        self.vid[v] = *k;
        *k += 1;

        for &(nxt, _) in self.graph.adjacency_list[v].clone().iter() {
            if nxt == p {
                continue;
            }

            self.head[nxt] = if nxt == self.graph.adjacency_list[v][0].0 {
                self.head[v]
            } else {
                nxt
            };

            self.dfs_2(nxt, v, k);
        }
    }

    /// 頂点の組 u, v に対してその最近共通祖先を計算する。
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.vid[u] > self.vid[v] {
                (u, v) = (v, u);
            }

            if self.head[u] == self.head[v] {
                return u;
            }

            v = self.parent[self.head[v]];
        }
    }
}
