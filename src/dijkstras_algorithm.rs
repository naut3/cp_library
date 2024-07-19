use crate::extended_number::{ExtendedNumber, HasMaxValue};
use crate::graph::Graph;

/// calculate shortest path from `source` and return sum of weight. If can't reach from `source`, return infinity.
pub fn dijkstras_algorithm<
    W: Default
        + HasMaxValue<S = W>
        + Into<ExtendedNumber<W>>
        + Clone
        + Copy
        + Ord
        + std::ops::Add<Output = W>,
>(
    graph: &dyn Graph<Weight = W>,
    source: usize,
) -> Vec<ExtendedNumber<W>> {
    assert!(source < graph.size());

    let mut dist = vec![Into::<ExtendedNumber<W>>::into(W::M); graph.size()];
    dist[source] = Into::<ExtendedNumber<W>>::into(W::default());
    let mut seen = vec![false; graph.size()];

    let mut hq = std::collections::BinaryHeap::default();
    hq.push((
        std::cmp::Reverse(Into::<ExtendedNumber<W>>::into(W::default())),
        source,
    ));

    while let Some((_, u)) = hq.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;

        for &(v, w) in graph.adjacent(u) {
            if !seen[v] {
                let d = dist[u] + Into::<ExtendedNumber<W>>::into(w);

                if d < dist[v] {
                    dist[v] = d;
                    hq.push((std::cmp::Reverse(d), v));
                }
            }
        }
    }

    return dist;
}
