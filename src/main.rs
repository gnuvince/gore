use std::io;
use std::io::Read;

extern crate gore;

use gore::token::Token;
use gore::scanner::Scanner;
use gore::error::Error;

fn main() {
    let mut stdin = io::stdin();
    let mut bytes = Vec::new();
    let _ = stdin.read_to_end(&mut bytes);
    let mut scanner = Scanner::new("<stdin>".to_string(), bytes);
    //scan(&mut scanner);
    match all_tokens(&mut scanner) {
        Ok(ref toks) => {
            for tok in toks {
                println!("{:?}", tok);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
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


fn all_tokens(scanner: &mut Scanner) -> Result<Vec<Token>, Error> {
    let mut toks = Vec::new();
    loop {
        match scanner.next() {
            Ok(tok) => {
                if tok.is_eof() { break; }
                toks.push(tok);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    return Ok(toks);
}
