use cp_library::lazy_segment_tree::{AddMax, AddMin, LazySegTree, UpdateMax, UpdateMin};
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[test]
fn test_minimal() {
    let mut stree: LazySegTree<UpdateMin<usize>> = LazySegTree::new(5);

    stree.insert(0, &1);
    assert_eq!(stree.get(0), 1);

    stree.apply(0..3, &10);
    assert_eq!(stree.get(0), 10);

    stree.apply(3..5, &100);
    assert_eq!(stree.prod(2..=3), 10);

    stree.apply(1..=3, &1000);
    assert_eq!(stree.prod(1..=2), 1000);

    stree.insert(1, &0);
    assert_eq!(stree.prod(1..=2), 0);
}

/// Range Minimum Query + Range Update Query で愚直解と結果が一致するかをテストする
#[test]
fn test_random_rminq_and_ruq() {
    let n = 5000;
    let q = 5000;

    let mut rng = thread_rng();

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n + 1);
        let mut r = rng.gen_range(0..n);

        if l >= r {
            (l, r) = (r, l - 1);
        }

        (l, r)
    };

    let mut a = (0..n)
        .map(|_| rng.gen_range(0..1 << 15))
        .collect::<Vec<_>>();
    let mut stree: LazySegTree<UpdateMin<usize>> = LazySegTree::new(n);

    for i in 0..n {
        stree.insert(i, &a[i]);
    }

    for _ in 0..q {
        if rng.gen() {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..r {
                a[i] = x;
            }

            stree.apply(l..r, &x);
        } else {
            let (l, r) = interval(&mut rng);

            let mut ret = usize::MAX;

            for i in l..r {
                ret = std::cmp::min(ret, a[i]);
            }

            assert_eq!(ret, stree.prod(l..r));
        }
    }
}

/// Range Maximum Query + Range Update Query で愚直解と結果が一致するかをテストする
#[test]
fn test_random_rmaxq_and_ruq() {
    let n = 5000;
    let q = 5000;

    let mut rng = thread_rng();

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n + 1);
        let mut r = rng.gen_range(0..n);

        if l >= r {
            (l, r) = (r, l - 1);
        }

        (l, r)
    };

    let mut a = (0..n)
        .map(|_| rng.gen_range(0..1 << 15))
        .collect::<Vec<_>>();
    let mut stree: LazySegTree<UpdateMax<usize>> = LazySegTree::new(n);

    for i in 0..n {
        stree.insert(i, &a[i]);
    }

    for _ in 0..q {
        if rng.gen() {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..r {
                a[i] = x;
            }

            stree.apply(l..r, &x);
        } else {
            let (l, r) = interval(&mut rng);

            let mut ret = usize::MIN;

            for i in l..r {
                ret = std::cmp::max(ret, a[i]);
            }

            assert_eq!(ret, stree.prod(l..r));
        }
    }
}

/// Range Minimum Query + Range Add Query で愚直解と結果が一致するかをテストする
#[test]
fn test_random_rminq_and_raq() {
    let n = 5000;
    let q = 5000;

    let mut rng = thread_rng();

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n + 1);
        let mut r = rng.gen_range(0..n);

        if l >= r {
            (l, r) = (r, l - 1);
        }

        (l, r)
    };

    let mut a = (0..n)
        .map(|_| rng.gen_range(0..1 << 15))
        .collect::<Vec<_>>();
    let mut stree: LazySegTree<AddMin<usize>> = LazySegTree::new(n);

    for i in 0..n {
        stree.insert(i, &a[i]);
    }

    for _ in 0..q {
        if rng.gen() {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..r {
                a[i] += x;
            }

            stree.apply(l..r, &x);
        } else {
            let (l, r) = interval(&mut rng);

            let mut ret = usize::MAX;

            for i in l..r {
                ret = std::cmp::min(ret, a[i]);
            }

            assert_eq!(ret, stree.prod(l..r));
        }
    }
}

/// Range Maximum Query + Range Add Query で愚直解と結果が一致するかをテストする
#[test]
fn test_random_rmaxq_and_raq() {
    let n = 5000;
    let q = 5000;

    let mut rng = thread_rng();

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n + 1);
        let mut r = rng.gen_range(0..n);

        if l >= r {
            (l, r) = (r, l - 1);
        }

        (l, r)
    };

    let mut a = (0..n)
        .map(|_| rng.gen_range(0..1 << 15))
        .collect::<Vec<_>>();
    let mut stree: LazySegTree<AddMax<usize>> = LazySegTree::new(n);

    for i in 0..n {
        stree.insert(i, &a[i]);
    }

    for _ in 0..q {
        if rng.gen() {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..r {
                a[i] += x;
            }

            stree.apply(l..r, &x);
        } else {
            let (l, r) = interval(&mut rng);

            let mut ret = usize::MIN;

            for i in l..r {
                ret = std::cmp::max(ret, a[i]);
            }

            assert_eq!(ret, stree.prod(l..r));
        }
    }
}
