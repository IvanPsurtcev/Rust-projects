pub mod func;
extern crate structopt;

use func::*;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use structopt::StructOpt;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Output file
	//#[structopt(default_value = ".", parse(from_os_str))] // ls -a
    #[structopt(default_value = ".", parse(from_os_str))] // ls -l
	path: PathBuf,

    #[structopt(short = "r", long = "recurse")]
    recurse: bool,

    #[structopt(short = "l", long = "l")]
    l: bool,

    #[structopt(short = "a", long = "a")]
    a: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("file path: {:?}", opt.path);
    if opt.a {
        if let Err(ref e) = ls_a::run(&opt.path) {
            println!("{}", e);
        }
    }

    if opt.recurse {
        if let Err(ref e) = ls_r::run(&opt.path) {
            println!("{}", e);
        }
    }

    if opt.l {
        if let Err(ref e) = ls_r::run(&opt.path) {
            println!("{}", e);
        }
    }
    
}

// fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
//     // if dir.is_dir() {
//     //     for entry in fs::read_dir(dir)? {
//     //         let entry = entry?;
//     //         let file_name = entry
//     //             .file_name()
//     //             .into_string()
//     //             .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

//     //         // добавить в иф условие с -h
//     //         // if точки нет &, тогда выполняю это, иначе ничего не делать
//     //         let metadata = entry.metadata()?;
//     //         let mode = metadata.permissions().mode();
//     //         let permissions = parse_permissions(mode as u32);
//     //         let size = metadata.len();
//     //         let modified = DateTime::<Local>::from(metadata.modified()?);
//     //         println!(
//     //             "{}, {:>5}, {}, {}", 
//     //             permissions,
//     //             size,
//     //             modified.format("%_d %b %H:%M").to_string(),
//     //             file_name
//     //         );
//     //     }
//     // }
//     // Ok(())
//     todo!()
// }

// fn ls_R(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
//     let dir_entries = read_dir(".").unwrap();
//     if dir.is_dir() {
        
//     }
// }

