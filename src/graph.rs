pub struct Graph<T> {
    pub size: usize,
    adjacency_list: Vec<Vec<(usize, T)>>,
}

impl<T: Clone> Graph<T> {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            adjacency_list: vec![vec![]; size],
        };
    }

    pub fn add_directed_edge(&mut self, from: usize, to: usize, weight: &T) {
        assert!(from < self.size && to < self.size);
        self.adjacency_list[from].push((to, weight.clone()));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: &T) {
        assert!(u < self.size && v < self.size);
        self.adjacency_list[u].push((v, weight.clone()));
        self.adjacency_list[v].push((u, weight.clone()));
    }

    pub fn adjacent(&self, v: usize) -> std::slice::Iter<'_, (usize, T)> {
        return self.adjacency_list[v].iter();
    }
}
