use crate::extended_number::ExtendedNumber;
use crate::graph::Graph;

/// calculate shortest path from `source` and return sum of weight. If can't reach from `source`, return infinity.
pub fn dijkstras_algorithm<
    W: ExtendedNumber<Item = T> + Clone + Copy + PartialOrd + Ord + std::ops::Add<Output = W>,
    T: Clone + Copy,
>(
    graph: &dyn Graph<Weight = W::Item>,
    source: usize,
) -> Vec<W> {
    assert!(source < graph.size());

    let mut dist = vec![W::inf(); graph.size()];
    dist[source] = W::zero();
    let mut seen = vec![false; graph.size()];

    let mut hq = std::collections::BinaryHeap::default();
    hq.push((std::cmp::Reverse(W::zero()), source));

    while let Some((_, u)) = hq.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;

        for &(v, w) in graph.adjacent(u) {
            if !seen[v] {
                let d = dist[u] + W::from(w);

                if d < dist[v] {
                    dist[v] = d;
                    hq.push((std::cmp::Reverse(d), v));
                }
            }
        }
    }

    return dist;
}
