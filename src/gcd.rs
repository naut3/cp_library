pub fn gcd<T: PartialOrd + Default + std::ops::Rem<Output = T> + Copy>(a: T, b: T) -> T {
    if a < b {
        return gcd(b, a);
    }

    if b == T::default() {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
