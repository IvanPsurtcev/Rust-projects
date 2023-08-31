pub mod func;

use func::hash_finder;
use std::env;


/// The main entry point of the application.
///
/// It parses the command line arguments for `N` and `F` parameters and invokes the hash finding process.
/// 
/// Usage:
/// 
/// ```
/// ./hashing -N <number of trailing zeros> -F <number of hashes>
/// ```
///
fn main() {
    let args: Vec<String> = env::args().collect();

    let n = args.iter().position(|x| x == "-N").map(|i| args[i + 1].parse::<usize>().unwrap()).unwrap_or(0);
    let f = args.iter().position(|x| x == "-F").map(|i| args[i + 1].parse::<usize>().unwrap()).unwrap_or(0);

    if n > 0 && f > 0 {
        let hashes = hash_finder::find_hashes(n, f);
        for (i, hash) in hashes {
            println!("{}, \"{}\"", i, hash);
        }
    } else {
        eprintln!("Usage: ./target/release/hashing -N <number of trailing zeros> -F <number of hashes>");
    }
}

#[cfg(test)]
mod tests {
    use super::hash_finder;

    #[test]
    fn test_find_hashes() {
        let hashes = hash_finder::find_hashes(2, 1);
        assert_eq!(hashes.len(), 1);
        assert!(hashes[0].1.ends_with("00"));

        let hashes = hash_finder::find_hashes(3, 1);
        assert_eq!(hashes.len(), 1);
        assert!(hashes[0].1.ends_with("000"));
    }

    #[test]
    fn test_find_multiple_hashes() {
        let hashes = hash_finder::find_hashes(2, 3);
        assert_eq!(hashes.len(), 3);
        for hash in &hashes {
            assert!(hash.1.ends_with("00"));
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_n() {
        let hashes = hash_finder::find_hashes(0, 3);
        assert_eq!(hashes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_f() {
        let hashes = hash_finder::find_hashes(3, 0);
        assert_eq!(hashes.len(), 0);
    }
}
