/**
 *@author Andrew Plaza
 */

use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::error::Error;

use self::err::ParserError;
use core::types::Emotifuck;
use emotifuck_parser::*;

/// Structs and Stuff
/// includes the generated code from PEG
#[allow(dead_code)]
pub mod emotifuck_grammar {
	include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
    types: Vec<Emotifuck>,
}

/// Runs the grammar on the source file
/// does any other needed transformations 
/// until it's passed to the interpreter
impl Parser {
	pub fn new(source_file: &str) -> Result<Parser, ParserError> {
	    let mut f = File::open(source_file)?;

        let mut source = String::new();
        f.read_to_string(&mut source)?;
        let source = emotifuck_grammar::content(source.as_ref())?;
        Ok(Parser{types: source})
	}
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.types)
    }
}

/// Tests for the parser
#[cfg(test)]
#[test]
fn test_parser() {
    let parser = match Parser::new("src/emotifuck_parser/emotifuck.test") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    println!("Parser: {}", parser.to_string());
}
