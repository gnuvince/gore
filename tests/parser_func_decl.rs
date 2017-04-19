extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn return_type() {
    assert!(parse_decl(b"func f() { }").is_ok());
    assert!(parse_decl(b"func f() int { }").is_ok());
    assert!(parse_decl(b"func f() []string { }").is_ok());
    assert!(parse_decl(b"func f() [8][8]float { }").is_ok());
    assert!(parse_decl(b"func f() + { }").is_err());
}

#[test]
fn param_list() {
    assert!(parse_decl(b"func f(x int) { }").is_ok());
    assert!(parse_decl(b"func f(x, y int) { }").is_ok());
    assert!(parse_decl(b"func f(x int, y int) { }").is_ok());
    assert!(parse_decl(b"func f(x int, y, z int) { }").is_ok());
    assert!(parse_decl(b"func f(x int, y int, a, b float64) { }").is_ok());
}
