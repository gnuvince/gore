extern crate gore;

use gore::error::GoreErrorType as ET;
use gore::scanner::Scanner;
use gore::parser::Parser;

pub fn parse_error(src: &[u8]) -> Result<(), ET> {
    let mut scanner = Scanner::new("<test>".to_string(), src.to_vec());
    let toks =
        match scanner.all_tokens() {
            Ok(tokens) => tokens,
            Err(gore_err) => {
                return Err(gore_err.ty);
            }
        };
    let mut parser = Parser::new(toks);
    match parser.parse() {
        Err(gore_err) => {
            Err(gore_err.ty)
        }
        Ok(_ast) => {
            Ok(())
        }
    }

}


pub fn parse_decl(decl: &[u8]) -> Result<(), ET> {
    let mut src = b"package main\n".to_vec();
    src.extend_from_slice(decl);
    parse_error(&src[..])
}
