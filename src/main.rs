use std::env;
use std::fs::File;
mod emotifuck_parser;
mod types;

fn main() {
    let args : Vec<String> = env::args().collect();
//    let mut f = File::open(args[1]).expect("File '{}' not found.", args[1]);
    //let mut contents = String::new();
    //f.read_to_string(&mut contents).expect("Something went wrong. #emotirekt");
}
