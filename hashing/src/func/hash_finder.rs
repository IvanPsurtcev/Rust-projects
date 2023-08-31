extern crate sha2;
extern crate rayon;

use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::sync::{Mutex, Arc};

/// Find hashes with the specified trailing zeros.
///
/// # Arguments
/// 
/// * `n` - The number of trailing zeros required in the hash.
/// * `f` - The number of hashes to be found.
///
/// # Returns
/// 
/// A vector containing pairs of number and its hash meeting the criteria.
///
pub fn find_hashes(n: usize, f: usize) -> Vec<(u64, String)> {
    let found = Arc::new(Mutex::new(Vec::new()));

    (1..).par_bridge().find_any(|i| {
        let mut hasher = Sha256::new();
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize();
        let hex = format!("{:x}", result);

        if hex.ends_with(&"0".repeat(n)) {
            let mut local_found = found.lock().unwrap();
            local_found.push((*i, hex.clone()));

            local_found.len() >= f
        } else {
            false
        }
    });

    Arc::try_unwrap(found).unwrap().into_inner().unwrap()
}