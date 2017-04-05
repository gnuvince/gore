extern crate gore;

use gore::error::GoreErrorType as ET;
use gore::scanner::Scanner;
use gore::parser::Parser;

pub fn parse_error(src: &[u8]) -> Option<ET> {
    let mut scanner = Scanner::new("<test>".to_string(), src.to_vec());
    let toks =
        match scanner.all_tokens() {
            Ok(tokens) => tokens,
            Err(gore_err) => {
                return Some(gore_err.ty);
            }
        };
    let mut parser = Parser::new(toks);
    match parser.parse() {
        Err(gore_err) => {
            Some(gore_err.ty)
        }
        Ok(_ast) => {
            None
        }
    }

}
