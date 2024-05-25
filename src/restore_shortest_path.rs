use crate::graph::Graph;

pub fn restore_shortest_path<
    W: Sized + std::ops::Add<Output = W> + PartialOrd + Ord + Default + Clone + Copy,
>(
    graph: &dyn Graph<Weight = W>,
    source: usize,
    destination: usize,
) -> Option<(W, Vec<usize>)> {
    let (dist, prev) = {
        let size = graph.size();
        let mut dist = vec![None; size];
        dist[source] = Some(W::default());
        let mut seen = vec![false; size];
        let mut prev = vec![usize::MAX; size];

        let mut hq = std::collections::BinaryHeap::default();
        hq.push((std::cmp::Reverse(W::default()), source));

        while let Some((_, u)) = hq.pop() {
            if u == destination {
                break;
            }

            if seen[u] {
                continue;
            }
            seen[u] = true;

            for &(v, w) in graph.adjacent(u).iter() {
                if !seen[v] {
                    let d = dist[u].unwrap() + w;

                    match dist[v] {
                        Some(pd) => {
                            if d < pd {
                                dist[v] = Some(d);
                                hq.push((std::cmp::Reverse(d), v));
                                prev[v] = u;
                            }
                        }
                        None => {
                            dist[v] = Some(d);
                            hq.push((std::cmp::Reverse(d), v));
                            prev[v] = u;
                        }
                    }
                }
            }
        }

        (dist, prev)
    };

    match dist[destination] {
        Some(d) => {
            let mut now = destination;
            let mut path = vec![destination];

            while now != source {
                path.push(prev[now]);
                now = prev[now];
            }

            path.reverse();
            return Some((d, path));
        }
        None => return None,
    }
}
