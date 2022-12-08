use std::fs;
use std::path::PathBuf;
use std::error::Error;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;


fn parse_permissions(mode: u32) -> String {
	let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
	let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
	let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
	[user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
	match (mode & read, mode & write, mode & execute) {
		(0, 0, 0) => "---",
		(_, 0, 0) => "r--",
		(0, _, 0) => "-w-",
		(0, 0, _) => "--x",
		(_, 0, _) => "r-x",
		(_, _, 0) => "rw-",
		(0, _, _) => "-wx",
		(_, _, _) => "rwx",
	}.to_string()
}

pub fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    //println!("dir: {:?}", dir);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            //println!("file: {:?}", entry);
            // добавить в иф условие с -h
            // if точки нет &, тогда выполняю это, иначе ничего не делать
            if file_name.chars().nth(0).unwrap() == '.' {
                continue;
            } else {
                let metadata = entry.metadata()?;
                let mode = metadata.permissions().mode();
                let permissions = parse_permissions(mode as u32);
                let size = metadata.len();
                let modified = DateTime::<Local>::from(metadata.modified()?);
                println!(
                    "{}, {:>5}, {}, {}", 
                    permissions,
                    size,
                    modified.format("%_d %b %H:%M").to_string(),
                    file_name
                );
            }
            
        }
    }
    Ok(())
}