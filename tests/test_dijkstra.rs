use cp_library::dijkstras_algorithm::dijkstras_algorithm;
use cp_library::extended_number::ExtNum;
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

    let dist: Vec<ExtNum<u32>> = dijkstras_algorithm(&graph, 0);

    assert_eq!(dist[0], ExtNum(0));
    assert_eq!(dist[1], ExtNum(1));
    assert_eq!(dist[2], ExtNum(11));
    assert_eq!(dist[3], ExtNum(111));
    assert_eq!(dist[4], ExtNum(1000));
    assert_eq!(format!("{}", dist[4]), "1000"); // format でいい感じに出力できる

    assert_eq!(dist[5], ExtNum::<u32>::inf()); // 到達不可能な場合は無限大扱いになる
    assert_eq!(format!("{}", dist[5]), "INF"); // INF と出力されるが、この辺は適当に処理したほうが好ましい
    assert!(dist[5].is_inf()); // 到達不可能(=inf)かどうかは判定関数を用意してある
}
