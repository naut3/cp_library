/// グラフ
pub trait Graph {
    type Weight;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Weight);
    fn adjacent(&self, v: usize) -> &[(usize, Self::Weight)];
}

/// 有向グラフ
#[derive(Clone)]
pub struct DirectedGraph<W> {
    pub size: usize,
    pub adjacency_list: Vec<Vec<(usize, W)>>,
}

impl<W: Clone> DirectedGraph<W> {
    /// 頂点数が size の有向グラフを新たに生成する
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            adjacency_list: vec![vec![]; size],
        };
    }

    /// u から v への重み w の有向辺を追加する
    pub fn add_edge(&mut self, u: usize, v: usize, w: W) {
        self.adjacency_list[u].push((v, w));
    }

    /// 頂点 v の隣接リストを返す
    pub fn adjacent(&self, v: usize) -> &[(usize, W)] {
        &self.adjacency_list[v]
    }
}

impl<W> Graph for DirectedGraph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.size
    }
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Weight) {
        self.adjacency_list[u].push((v, w));
    }
    fn adjacent(&self, v: usize) -> &[(usize, W)] {
        &self.adjacency_list[v]
    }
}

/// 無向グラフ
#[derive(Clone)]
pub struct UndirectedGraph<W> {
    pub size: usize,
    pub adjacency_list: Vec<Vec<(usize, W)>>,
}

impl<W: Clone> UndirectedGraph<W> {
    /// 頂点数が size の無向グラフを新たに生成する
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            adjacency_list: vec![vec![]; size],
        };
    }

    /// u と v の間に重み w の辺を追加する
    pub fn add_edge(&mut self, u: usize, v: usize, w: W) {
        self.adjacency_list[u].push((v, w.clone()));
        self.adjacency_list[v].push((u, w));
    }

    /// 頂点 v の隣接リストを返す
    pub fn adjacent(&self, v: usize) -> &[(usize, W)] {
        &self.adjacency_list[v]
    }
}

impl<W: Clone> Graph for UndirectedGraph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.size
    }
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Weight) {
        self.adjacency_list[u].push((v, w.clone()));
        self.adjacency_list[v].push((u, w));
    }
    fn adjacent(&self, v: usize) -> &[(usize, W)] {
        &self.adjacency_list[v]
    }
}
