use cp_library::dijkstras_algorithm::dijkstras_algorithm;
use cp_library::extended_number::ExtendedNumber;
use cp_library::graph::DirectedGraph;

#[test]
fn test_small() {
    // 型まわりがあまりいい感じになっていないので、その辺を例示するテスト
    let size = 6;
    let edges = [
        (0, 1, 1u32),
        (1, 2, 10),
        (2, 3, 100),
        (3, 4, 1000),
        (0, 4, 1000),
    ];

    let graph = {
        let mut graph = DirectedGraph::new(size);

        for (u, v, w) in edges {
            graph.add_edge(u, v, w);
        }

        graph
    };

    let dist = dijkstras_algorithm(&graph, 0);

    assert_eq!(dist[0], ExtendedNumber(0));
    assert_eq!(dist[1], ExtendedNumber(1));
    assert_eq!(dist[2], ExtendedNumber(11));
    assert_eq!(dist[3], ExtendedNumber(111));
    assert_eq!(dist[4], ExtendedNumber(1000));
    assert_eq!(format!("{}", dist[4]), "1000"); // format でいい感じに出力できる

    assert_eq!(dist[5], ExtendedNumber::<u32>::INF); // 到達不可能な場合は無限大扱いになる
    assert_eq!(format!("{}", dist[5]), "INF"); // INF と出力されるが、この辺は適当に処理したほうが好ましい
    assert!(dist[5].is_inf()); // 到達不可能(=inf)かどうかは判定関数を用意してある
}
