use cp_library::monoid::{Add, Max, Min};
use cp_library::segment_tree::SegmentTree;

#[test]
#[allow(unused_variables)]
fn test_generate_segtree() {
    let stree: SegmentTree<Add<u64>> = SegmentTree::new(5);
    let stree: SegmentTree<Max<isize>> = SegmentTree::new(1000);
    let stree: SegmentTree<Min<usize>> = SegmentTree::new(1);
}

#[test]
fn test_update_value() {
    let mut stree: SegmentTree<Add<isize>> = SegmentTree::new(10);
    stree.update(0, 1);
    assert_eq!(stree.get(0), 1);
    stree.update(4, -1);
    assert_eq!(stree.get(4), -1);
}

#[test]
fn test_prod() {
    let mut stree: SegmentTree<Add<isize>> = SegmentTree::new(5);
    assert_eq!(stree.prod(..), 0);

    stree.update(0, 1);
    stree.update(1, 10);
    stree.update(4, 100);

    assert_eq!(stree.prod(0..=1), 11);
    assert_eq!(stree.prod(1..), 110);
    assert_eq!(stree.prod(4..5), 100);
    assert_eq!(stree.prod(..1), 1);

    assert_eq!(format!("{}", stree), "1 10 0 0 100");
}

#[test]
fn test_from_vec() {
    let a = vec![100000isize, 10000, 1000, 100, 10, 1];
    let stree: SegmentTree<Add<isize>> = SegmentTree::from(&a);

    assert_eq!(stree.prod(0..=1), 110000);
    assert_eq!(stree.prod(..), 111111);
    assert_eq!(stree.prod(2..=5), 1111);
    assert_eq!(stree.prod(1..4), 11100);

    assert_eq!(format!("{}", stree), "100000 10000 1000 100 10 1");
}
