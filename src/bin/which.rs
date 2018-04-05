extern crate which;
extern crate clap;

use clap::{Arg, App};

fn main() {

    let matches = App::new("which")
        .version("2.0.0")
        .arg(Arg::from_usage("[NAME] 'binary name to find'"))
        .get_matches();

    let name = matches.value_of("NAME").expect("None value of [NAME]");
 
    let path = which::which(name).unwrap();
    println!("{}", path.canonicalize().unwrap().to_str().unwrap());
}