extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn type_decl_ok() {
    assert!(parse_decl(b"type num int").is_ok());
    assert!(parse_decl(b"type reals []float64").is_ok());
    assert!(parse_decl(b"type (re float64; im float64;)").is_ok());
    assert!(parse_decl(b"type (
re float64
im float64
)").is_ok());
}

#[test]
fn type_decl_error() {
    assert!(parse_decl(b"type").is_err());
    assert!(parse_decl(b"type real").is_err());
    assert!(parse_decl(b"type ()").is_err());
    assert!(parse_decl(b"type (num)").is_err());
    assert!(parse_decl(b"type (num int; real)").is_err());
    assert!(parse_decl(b"type (42)").is_err());
}
