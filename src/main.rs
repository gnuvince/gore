use std::io;
use std::io::Read;

extern crate gore;

use gore::token::Token;
use gore::scanner::Scanner;
use gore::error::GoreError;
use gore::parser::Parser;

fn main() {
    let mut stdin = io::stdin();
    let mut bytes = Vec::new();
    let _ = stdin.read_to_end(&mut bytes);
    let mut scanner = Scanner::new("<stdin>".to_string(), bytes);
    scan(&mut scanner);

    /*
    match all_tokens(&mut scanner) {
        Ok(toks) => {
            let mut parser = Parser::new(toks);
            println!("{:#?}", parser.parse());
        }
        Err(err) => {
            println!("{}", err);
        }
    }
     */
}


fn scan(scanner: &mut Scanner) {
    loop {
        match scanner.next() {
            Ok(tok) => {
                if tok.is_eof() { break; }
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }
}
