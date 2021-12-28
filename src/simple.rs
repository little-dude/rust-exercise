use std::collections::HashSet;

pub fn remove_duplicates(values: Vec<u8>) -> Vec<u8> {
    let mut duplicates: HashSet<u8> = HashSet::new();
    values
        .into_iter()
        .filter(|item| duplicates.insert(*item))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            remove_duplicates(vec![5, 3, 9, 0, 3, 5, 5]),
            vec![5, 3, 9, 0]
        );
    }
}
