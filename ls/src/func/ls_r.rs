use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::error::Error;

fn is_hidden(entry: &DirEntry) -> bool {
    if entry.path().is_dir() {
        false
    } else {
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
}

pub fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let walker = WalkDir::new(dir).into_iter();
    println!("walker {:?}", walker);
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        println!("{}", entry?.path().display());
    }
    Ok(())
}