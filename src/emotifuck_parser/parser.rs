/**
 *@author Andrew Plaza
 */
 use types::Emotifuck;

/// Structs and Stuff
/// includes the generated code from PEG

use emotifuck_parser::*;

#[allow(dead_code)]
pub mod emotifuck_grammer {
	include!(concat!(env!("OUT_DIR"), "/emotifuck_grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
	emotifuckTypes: Vec<Emotifuck>,
}

impl Parser {
	pub fn new(source_file: &str) -> Result<Parser, ParseError> {
		
	}
}

#[cfg(test)]
#[test]
fn test_parser() {
    

}
