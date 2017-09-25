/**
 *@author Andrew Plaza <andrew.plaza@protonmail.com>
 *@author Sean Batzel <romulus108@protonmail.com>
 */

extern crate colored;
mod core;
mod interpreter;
mod emotifuck_parser;

use std::env;
use std::process;
use std::error::Error;
use colored::Colorize;
use interpreter::interpreter::{compile, interpret};
use emotifuck_parser::parser::Parser;

fn main() {
    let mut args = env::args_os();
    /// Parser returns a Parser { Vec<Emotifuck> }
    // println!("{:?}", args);
    let parser;
    args.next();
    if let Some(source) = args.next() {
        // println!("{:?}", source);
        parser = match Parser::new(&source.into_string().unwrap()) {
            Ok(x) => x,
            Err(e) => {
                println!("Error!: {}", e.description().red().bold());
                process::exit(1);
            }
        };
        let target = compile(parser.types);
        interpret(target);
    }
}
