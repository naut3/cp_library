use cp_library::wavelet_matrix::{BitVector, WaveletMatrix};
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[test]
fn test_bitvector() {
    let a = vec![0b111, 0b10];
    let bv = BitVector::from(&a);

    assert_eq!(bv.access(0), true);
    assert_eq!(bv.access(1), true);
    assert_eq!(bv.access(2), true);
    assert_eq!(bv.access(3), false);
    assert_eq!(bv.access(64), false);
    assert_eq!(bv.access(65), true);

    assert_eq!(bv.rank(10, true), 3);
    assert_eq!(bv.rank(65, true), 4);
    assert_eq!(bv.rank(63, true), 3);
    assert_eq!(bv.rank(2, false), 0);
    assert_eq!(bv.rank(3, false), 1);
    assert_eq!(bv.rank(64, false), 62);

    //eprintln!("{}", bv);
}

#[test]
fn test_wm() {
    let a = vec![5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
    let wm = WaveletMatrix::from(&a, 4);

    for i in 0..12 {
        assert_eq!(wm.access(i), a[i], "{i} failed");
    }

    assert_eq!(wm.quantile(0, 6, 0), 1);
    assert_eq!(wm.range_freq(0, 5, 5), 2);
}

#[test]
fn test_wm_random_access() {
    let mut rng = thread_rng();

    let a = (0..5000)
        .map(|_| rng.gen_range(0..1 << 60))
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::from(&a, 62);

    for i in 0..5000 {
        assert_eq!(wm.access(i), a[i]);
    }
}

#[test]
fn test_wm_random_quantile() {
    let mut rng = thread_rng();

    let n = 2500;

    let a = (0..n)
        .map(|_| rng.gen_range(0..1 << 60))
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::from(&a, 62);

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n - 1);
        let mut r = rng.gen_range(0..n);
        if l >= r {
            (l, r) = (r, l + 1);
        }

        (l, r)
    };

    for _ in 0..n {
        let (l, r) = interval(&mut rng);
        let k = rng.gen_range(0..=r - l - 1);

        let mut b = a[l..r].to_vec();
        b.sort();

        assert_eq!(wm.quantile(l, r, k), b[k]);
    }
}

#[test]
fn test_wm_random_range_freq() {
    let mut rng = thread_rng();

    let n = 2500;

    let a = (0..n)
        .map(|_| rng.gen_range(0..1 << 30))
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::from(&a, 60);

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n - 1);
        let mut r = rng.gen_range(0..n);
        if l >= r {
            (l, r) = (r, l + 1);
        }

        (l, r)
    };

    for _ in 0..n {
        let (l, r) = interval(&mut rng);
        let upper = rng.gen_range(0..1 << 30);

        let mut cnt = 0;
        for i in l..r {
            if a[i] < upper {
                cnt += 1;
            }
        }

        assert_eq!(wm.range_freq(l, r, upper), cnt);
    }
}

#[test]
fn test_wm_random_range_sum() {
    let mut rng = thread_rng();

    let n = 2500;

    let a = (0..n)
        .map(|_| {
            let a = rng.gen_range(0..1 << 30);
            (a, a)
        })
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::from_weighted(&a, 60);

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n - 1);
        let mut r = rng.gen_range(0..n);
        if l >= r {
            (l, r) = (r, l + 1);
        }

        (l, r)
    };

    for _ in 0..n {
        let (l, r) = interval(&mut rng);
        let upper = rng.gen_range(0..1 << 30);

        let mut cnt = 0;
        for i in l..r {
            if a[i].0 < upper {
                cnt += a[i].0;
            }
        }

        assert_eq!(wm.range_sum(l, r, upper), cnt);
    }
}

#[test]
fn test_wm_random_sum() {
    let mut rng = thread_rng();

    let n = 2500;

    let a = (0..n)
        .map(|_| {
            let a = rng.gen_range(0..1 << 30);
            (a, a)
        })
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::from_weighted(&a, 60);

    let interval = |rng: &mut ThreadRng| {
        let mut l = rng.gen_range(0..n - 1);
        let mut r = rng.gen_range(0..n);
        if l >= r {
            (l, r) = (r, l + 1);
        }

        (l, r)
    };

    for _ in 0..n {
        let (l, r) = interval(&mut rng);

        let mut cnt = 0;
        for i in l..r {
            cnt += a[i].0;
        }

        assert_eq!(wm.sum(l, r), cnt);
    }
}
