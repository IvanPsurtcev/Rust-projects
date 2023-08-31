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
