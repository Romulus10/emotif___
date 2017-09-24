/**
 *@author Andrew Plaza
 */

use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::error::Error;

use emotifuck_parser::err::ParserError;
use types::Emotifuck;


/// Structs and Stuff
/// includes the generated code from PEG

#[allow(dead_code)]
pub mod emotifuck_grammar {
	include!(concat!(env!("OUT_DIR"), "/emotifuck_grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
    types: Vec<Emotifuck>,
}

/// Runs the grammar on the source file
/// does any other needed transformations 
/// until it's passed to the interpreter
impl Parser {
	pub fn new(source_file: &str) -> Result<Parser, ParseError> {
	    let mut f = File::open(config_file)?;

        let mut config = String::new();
        f.read_to_string(&mut config)?;
        let config = emotifuck_grammar::content(config.as_ref())?;
        
        let mut types = Vec::new();
        
        config.iter().map(|x| types.push(x));
        Ok(Parser{types})
	}
}

#[cfg(test)]
#[test]
fn test_parser() {
    

}
