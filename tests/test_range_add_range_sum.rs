use cp_library::lazy_segment_tree::{AddSum, LazySegTree, S};

#[test]
fn test_minimal() {
    let mut stree: LazySegTree<AddSum<isize>> = LazySegTree::new(5);

    // 最初に以下のように初期化しないと壊れるので注意すること。
    for i in 0..5 {
        stree.insert(i, &S::from(0));
    }

    assert_eq!(stree.prod(0..5).value(), 0);

    stree.apply(0..=1, &100);
    stree.apply(2..=3, &10);
    // [100, 100, 10, 10, 0]

    assert_eq!(stree.prod(1..=2).value(), 110);
    assert_eq!(stree.prod(0..=3).value(), 220);

    stree.apply(4..=4, &1000);
    // [100, 100, 10, 10, 1000]

    assert_eq!(stree.prod(3..=4).value(), 1010);

    stree.apply(1..=2, &10000);
    // [100, 10100, 10010, 10, 1000]

    assert_eq!(stree.prod(0..=1).value(), 10200);
}
