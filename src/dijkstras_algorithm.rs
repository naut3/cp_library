use crate::graph::Graph;

pub fn dijkstras_algorithm<
    W: Sized + std::ops::Add<Output = W> + PartialOrd + Ord + Default + Clone + Copy + 'static,
>(
    graph: &dyn Graph<Weight = W>,
    source: usize,
) -> Vec<Option<W>> {
    assert!(source < graph.size());

    let mut dist = vec![None; graph.size()];
    dist[source] = Some(W::default());
    let mut seen = vec![false; graph.size()];

    let mut hq = std::collections::BinaryHeap::default();
    hq.push((std::cmp::Reverse(W::default()), source));

    while let Some((_, u)) = hq.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;

        for &(v, w) in graph.adjacent(u) {
            if !seen[v] {
                let d = dist[u].unwrap() + w;

                match dist[v] {
                    Some(pd) => {
                        if d < pd {
                            dist[v] = Some(d);
                            hq.push((std::cmp::Reverse(d), v));
                        }
                    }
                    None => {
                        dist[v] = Some(d);
                        hq.push((std::cmp::Reverse(d), v));
                    }
                }
            }
        }
    }

    return dist;
}
