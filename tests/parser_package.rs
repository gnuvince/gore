extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::parse_error;


#[test]
fn test_package() {
    assert!(parse_error(b"").is_err());
    assert!(parse_error(b"main").is_err());
    assert!(parse_error(b"package").is_err());
    assert!(parse_error(b"package main").is_ok());
}
