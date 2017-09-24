extern crate peg;

/**
 * @author Andrew Plaza
 * A simple grammar for Emotifuck
 *
 */

fn main() {
    peg::cargo_build("src/emotifuck_parser/grammar.rustpeg");
}


