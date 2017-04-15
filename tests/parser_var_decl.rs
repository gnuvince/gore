extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn var_decl_ok() {
    assert!(parse_decl(b"var x int").is_ok());
    assert!(parse_decl(b"var x, y float64").is_ok());
    assert!(parse_decl(b"var x, y, z []string").is_ok());

    assert!(parse_decl(b"var x = a").is_ok());
    assert!(parse_decl(b"var x, y = a, b").is_ok());
    assert!(parse_decl(b"var x, y, z = a, b, c").is_ok());

    assert!(parse_decl(b"var x int = a").is_ok());
    assert!(parse_decl(b"var x, y int = a, b").is_ok());
    assert!(parse_decl(b"var x, y, z int = a, b, c").is_ok());

    assert!(parse_decl(b"var (x int; y float64;)").is_ok());
    assert!(parse_decl(b"var (x, y int; z float64;)").is_ok());
    assert!(parse_decl(b"var (x int; y, z float64;)").is_ok());

    assert!(parse_decl(b"var (x = a; y = b;)").is_ok());
    assert!(parse_decl(b"var (x, y = a, b; z = c;)").is_ok());
    assert!(parse_decl(b"var (x = a; y, z = b, c;)").is_ok());

    assert!(parse_decl(b"var (x int = a; y float64 = b;)").is_ok());
    assert!(parse_decl(b"var (x, y string = a, b; z []int = c;)").is_ok());
    assert!(parse_decl(b"var (x bool = a; y, z rune = b, c;)").is_ok());
}

#[test]
fn var_decl_err() {
    assert!(parse_decl(b"var").is_err());
    assert!(parse_decl(b"var x").is_err());
    assert!(parse_decl(b"var ()").is_err());
    assert!(parse_decl(b"var x = a, b").is_err());
    assert!(parse_decl(b"var x, y = a").is_err());
}
