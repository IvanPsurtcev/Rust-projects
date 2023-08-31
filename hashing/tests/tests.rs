#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hashes() {
        let hashes = find_hashes(2, 1);
        assert_eq!(hashes.len(), 1);
        assert!(hashes[0].1.ends_with("00"));

        let hashes = find_hashes(3, 1);
        assert_eq!(hashes.len(), 1);
        assert!(hashes[0].1.ends_with("000"));
    }

    #[test]
    fn test_find_multiple_hashes() {
        let hashes = find_hashes(2, 3);
        assert_eq!(hashes.len(), 3);
        for hash in &hashes {
            assert!(hash.1.ends_with("00"));
        }
    }

    #[test]
    fn test_zero_hashes() {
        let hashes = find_hashes(10, 1);
        assert_eq!(hashes.len(), 0);
    }

    #[test]
    fn test_invalid_n() {
        let hashes = find_hashes(0, 3);
        assert_eq!(hashes.len(), 0);
    }

    #[test]
    fn test_invalid_f() {
        let hashes = find_hashes(3, 0);
        assert_eq!(hashes.len(), 0);
    }
}
