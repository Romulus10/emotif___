/**
 *@author Andrew Plaza
 *
 */
use std::io;
use std::error;
use std::fmt;

use emotifuck_parser::parser::emotifuck_grammar::ParseError;

/// Possible Errors when parsing
/// Subject to change

#[derive(Debug)]
pub struct 404EmojiNotFound {
	pub v: String,
}

pub type 404EmojiNotFoundError = 404EmojiNotFound;

impl fmt::Display for 404EmojiNotFoundError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.v);
	}
}

impl error::Error for 404EmojiNotFoundError {
	fn description(&self) -> &str {
		"The Symbol Could Not Be Found"
	}
}

#[derive(Debug)]
pub enum ParserError {
	CouldNotOpenSourceCode(io::Error),
	CouldNotParseFile(ParseError),
	SymbolNotFound(404EmojiNotFoundError),
}

impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ParserError::CouldNotOpenSourceCode(ref err) => {
				write!(f, "Could not open Source Code File! {}", err)
			}
			ParserError::CouldNotParseFile(ref err) => {
				write(f, "Was not able to parse Emotifuck {}", err)
			}
			ParserError::SymbolNotFound(ref err) => {
				write(f, "Could Not Find Symbol {}", err)
			}
		}
	}
}

impl error::Error for ParserError {
	fn description(&self) -> &str {
		match *self {
		    ParserError::CouldNotOpenSourceCode(ref err) => err.description(),
            ParserError::CouldNotParseFile(ref err) => err.description(),
            ParserError::SymbolNotFound(ref err) => err.description(),
		}
	}

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParserError::CouldNotOpenSourceCode(ref err) => err.description(),
            ParserError::CouldNotParseFile(ref err) => err.description(),
            ParserError::SymbolNotFound(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> ParserError {
        ParserError::CouldNotOpenSourceCode(err)
    }
}

impl From<ParseError> for ParserError {
    fn from(err: ParseError) -> ParserError {
        ParserError::CouldNotParseFile(err)
    }
}

impl From<SymbolNotFoundError> for ParserError {
    fn from(err: SymbolNotFoundError) -> ParserError {
        ParserError::SymbolNotFound(err)
    }
}


