use std::collections::HashSet;

pub fn remove_duplicates(values: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    let mut duplicates: HashSet<u8> = HashSet::new();
    values.filter(move |item| duplicates.insert(*item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            remove_duplicates(vec![5, 3, 9, 0, 3, 5, 5].into_iter()).collect::<Vec<_>>(),
            vec![5, 3, 9, 0]
        );
    }
}
