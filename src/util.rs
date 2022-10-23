pub fn s2a<T: Copy + Sized, const N: usize>(s: &[T]) -> Option<[T; N]> {
    if s.len() != N {
        None
    } else {
        Some(std::array::from_fn(|idx| s[idx]))
    }
}