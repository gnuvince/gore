use std::io;
use std::io::Read;

extern crate gore;

use gore::scanner::Scanner;

fn main() {
    let mut stdin = io::stdin();
    let mut bytes = Vec::new();
    let _ = stdin.read_to_end(&mut bytes);
    let mut scanner = Scanner::new("-".to_string(), bytes);

    loop {
        match scanner.next() {
            Ok(tok) => {
                println!("{:?}", tok);
                if tok.is_eof() { break; }
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
}
