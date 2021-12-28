use std::{collections::HashSet, hash::Hash};

pub fn remove_duplicates<T>(values: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: Eq + Hash + Clone,
{
    let mut duplicates: HashSet<T> = HashSet::new();
    // NOTE: we could avoid cloning for existing duplicates
    values.filter(move |item| duplicates.insert(item.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            remove_duplicates(vec![5, 3, 9, 0, 3, 5, 5].into_iter()).collect::<Vec<u8>>(),
            vec![5, 3, 9, 0]
        );
    }
}
