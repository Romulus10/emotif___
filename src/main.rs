use std::env;
use std::io::Read;
use std::fs::File;

mod interpreter;

use interpreter::interpreter::{compile, interpret};

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File  not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong. #emotirekt");
}
