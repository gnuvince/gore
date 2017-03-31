use std::io;
use std::io::Read;

extern crate gore;

use gore::scanner::Scanner;

fn main() {
    let mut stdin = io::stdin();
    let mut bytes = Vec::new();
    let _ = stdin.read_to_end(&mut bytes);
    let mut scanner = Scanner::new("-".to_string(), bytes);
    scan(&mut scanner);
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
