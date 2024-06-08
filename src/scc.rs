use crate::graph::{DirectedGraph, Graph};

impl<T: Copy> DirectedGraph<T> {
    fn rev(&self) -> Self {
        let mut graph = DirectedGraph::new(self.size);

        for i in 0..self.size {
            for &(v, w) in self.adjacent(i) {
                graph.add_edge(v, i, w);
            }
        }

        return graph;
    }
}

/// 強連結成分分解
pub fn strongly_connected_components<T: Clone + Copy>(graph: DirectedGraph<T>) -> Vec<usize> {
    struct SCC<T> {
        graph: DirectedGraph<T>,
        seen: Vec<bool>,
        stop: Vec<usize>,
    }

    impl<T: Clone> SCC<T> {
        fn new(graph: &DirectedGraph<T>) -> Self {
            return Self {
                graph: graph.clone(),
                seen: vec![false; graph.size],
                stop: vec![],
            };
        }

        fn dfs(&mut self, u: usize) {
            self.seen[u] = true;

            for &(v, _) in self.graph.adjacency_list[u].clone().iter() {
                if self.seen[v] {
                    continue;
                }

                self.dfs(v);
            }

            self.stop.push(u);
        }
    }

    let size = graph.size;
    let mut scc = SCC::new(&graph);

    for i in 0..size {
        if !scc.seen[i] {
            scc.dfs(i);
        }
    }

    let graph_inv = graph.rev();
    let mut scc_inv = SCC::new(&graph_inv);

    let mut stop = scc.stop;
    stop.reverse();

    let mut ret = vec![0; size];
    let mut cnt = 0;

    for u in stop {
        if !scc_inv.seen[u] {
            scc_inv.dfs(u);

            for &v in scc_inv.stop.iter() {
                ret[v] = cnt;
            }

            scc_inv.stop.clear();
            cnt += 1;
        }
    }

    return ret;
}
