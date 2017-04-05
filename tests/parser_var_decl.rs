extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::parse_error;

fn parse_var_decl(decl: &[u8]) -> Option<ET> {
    let mut src = b"package main\n".to_vec();
    src.extend_from_slice(decl);
    parse_error(&src[..])
}

#[test]
fn var_decl_ok() {
    assert_eq!(None, parse_var_decl(b"var x int"));
    assert_eq!(None, parse_var_decl(b"var x, y float64"));
    assert_eq!(None, parse_var_decl(b"var x, y, z []string"));

    assert_eq!(None, parse_var_decl(b"var x = a"));
    assert_eq!(None, parse_var_decl(b"var x, y = a, b"));
    assert_eq!(None, parse_var_decl(b"var x, y, z = a, b, c"));

    assert_eq!(None, parse_var_decl(b"var x int = a"));
    assert_eq!(None, parse_var_decl(b"var x, y int = a, b"));
    assert_eq!(None, parse_var_decl(b"var x, y, z int = a, b, c"));
}

#[test]
fn var_decl_err() {
    assert_eq!(Some(ET::InvalidVarDecl), parse_var_decl(b"var"));
    assert_eq!(Some(ET::InvalidVarDecl), parse_var_decl(b"var x"));
    assert_eq!(Some(ET::VarExprLengthMismatch), parse_var_decl(b"var x = a, b"));
    assert_eq!(Some(ET::VarExprLengthMismatch), parse_var_decl(b"var x, y = a"));
}
