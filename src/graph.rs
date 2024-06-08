/// グラフ
pub trait Graph {
    type Weight;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Weight);
    fn adjacent(&self, v: usize) -> &Vec<(usize, Self::Weight)>;
}

/// 有向グラフ
#[derive(Clone)]
pub struct DirectedGraph<W> {
    pub size: usize,
    pub adjacency_list: Vec<Vec<(usize, W)>>,
}

impl<W: Clone> DirectedGraph<W> {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            adjacency_list: vec![vec![]; size],
        };
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
    fn adjacent(&self, v: usize) -> &Vec<(usize, W)> {
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
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            adjacency_list: vec![vec![]; size],
        };
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
    fn adjacent(&self, v: usize) -> &Vec<(usize, W)> {
        &self.adjacency_list[v]
    }
}
