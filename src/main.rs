/**
 *@author Andrew Plaza <andrew.plaza@protonmail.com>
 *@author Sean Batzel <romulus108@protonmail.com>
 */

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate colored;

mod core;
mod interpreter;
mod emotifuck_parser;

use std::env;
use std::process;
use std::error::Error;
use colored::Colorize;
use docopt::Docopt;

use interpreter::interpreter::{compile, interpret};
use emotifuck_parser::parser::Parser;

// Docopt usage string
const USAGE: &'static str = "
Emotif*ck Command Line Tool \u{1F917}
Usage: ./emotif___ <source>";

#[derive(Debug, Deserialize)]
struct Args {
    arg_source: String,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
     
    let parser = match Parser::new(&args.arg_source) {
        Ok(x) => x,
        Err(e) => {
            println!("Error!: {}, {}", e.description().red().bold(), e.to_string());
            process::exit(1);
        }
    };

    let target = compile(parser.types);
    interpret(target);
}
