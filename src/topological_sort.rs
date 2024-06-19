use crate::graph::DirectedGraph;

pub fn topological_sort<T: Clone>(graph: &DirectedGraph<T>) -> Vec<usize> {
    let size = graph.size;
    let mut out_deg = vec![0; size];
    let mut order = vec![];

    let mut graph_inv = DirectedGraph::new(size);

    for i in 0..size {
        for &(v, _) in graph.adjacent(i) {
            graph_inv.add_edge(v, i, ());
            out_deg[i] += 1;
        }
    }

    let mut q = std::collections::VecDeque::new();

    for i in 0..size {
        if out_deg[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(u) = q.pop_front() {
        order.push(u);

        for &(v, _) in graph_inv.adjacent(u) {
            out_deg[v] -= 1;
            if out_deg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    order.reverse();
    order
}
