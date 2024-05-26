pub fn coordinate_compression<T: std::cmp::Ord + Clone + Copy>(
    values: &[T],
) -> (Vec<T>, std::collections::BTreeMap<T, usize>) {
    let s: std::collections::BTreeSet<T> = values.iter().cloned().collect();
    let z: Vec<T> = s.iter().cloned().collect();
    let zinv: std::collections::BTreeMap<T, usize> =
        z.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    (z, zinv)
}
