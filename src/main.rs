extern crate colored;
mod core;
mod interpreter;
mod emotifuck_parser;

use std::env;
use std::io::Read;
use std::fs::File;
use std::process;
use std::error::Error;
use colored::Colorize;
use interpreter::interpreter::{compile, interpret};
use emotifuck_parser::parser::Parser;

fn main() {
    let mut args = env::args_os();
    /// Parser returns a Parser { Vec<Emotifuck> }
    println!("{:?}", args);
    let parser;
    if let Some(source) = args.next() {
        parser = match Parser::new(&source.into_string().unwrap()) {
            Ok(x) => x,
            Err(e) => {
                println!("Error!: {}", e.description().red().bold());
                process::exit(1);
            }
        };
    }
}
