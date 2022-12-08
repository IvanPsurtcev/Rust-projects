pub mod func;
extern crate structopt;

use func::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Output file
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
        if let Err(ref e) = ls_l::run(&opt.path) {
            println!("{}", e);
        }
    }
}


