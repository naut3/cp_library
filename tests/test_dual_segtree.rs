use cp_library::dual_segment_tree::DualSegTree;
use rand::{rngs::ThreadRng, thread_rng, Rng};

/// Range Add Point Get で愚直解と結果が一致するかをテストする
#[test]
fn test_additive_random() {
    use cp_library::monoid::Add;

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

    let mut a = vec![0; n];
    let mut stree: DualSegTree<Add<isize>> = DualSegTree::new(5000);

    for i in 0..n {
        let x = rng.gen_range(0..1 << 15);
        a[i] = x;
        stree.act(i..=i, &x);
    }

    for _ in 0..q {
        if rng.gen() {
            let i = rng.gen_range(0..n);
            assert_eq!(a[i], stree.get(i));
        } else {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..=r {
                a[i] += x;
            }

            stree.act(l..=r, &x);
        }
    }
}

/// Range Update Point Get で愚直解と結果が一致するかをテストする
#[test]
fn test_update_random() {
    use cp_library::monoid::Update;

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

    let mut a = vec![0; n];
    let mut stree: DualSegTree<Update<isize>> = DualSegTree::new(5000);

    for i in 0..n {
        let x = rng.gen_range(0..1 << 15);
        a[i] = x;
        stree.act(i..=i, &x);
    }

    for _ in 0..q {
        if rng.gen() {
            let i = rng.gen_range(0..n);
            assert_eq!(a[i], stree.get(i));
        } else {
            let (l, r) = interval(&mut rng);
            let x = rng.gen_range(0..1 << 15);

            for i in l..=r {
                a[i] = x;
            }

            stree.act(l..=r, &x);
        }
    }
}
