/**
 *@author Andrew Plaza
 */

/// Structs and Stuff
/// includes the generated code from PEG
/// 

use emotifuck_parser::*;

#[allow(dead_code)]
pub mod emotifuck_grammer {
	include!(concat!(env!("OUT_DIR"), "/emotifuck_grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
	
}