use itertools::Itertools;

pub fn iter_is_sorted<T: PartialOrd + Clone>(iter: impl Iterator<Item = T>) -> bool {
    iter.tuple_windows().all(|(l, r)| r >= l)
}

#[cfg(test)]
mod tests {
    use super::iter_is_sorted;

    #[test]
    fn is_sorted() {
        assert!(iter_is_sorted([0; 0].iter()));
        assert!(iter_is_sorted([1].iter()));
        assert!(iter_is_sorted([1, 2].iter()));
        assert!(iter_is_sorted([1, 2, 3].iter()));
    }
}
