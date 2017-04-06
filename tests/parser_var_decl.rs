extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn var_decl_ok() {
    assert_eq!(None, parse_decl(b"var x int"));
    assert_eq!(None, parse_decl(b"var x, y float64"));
    assert_eq!(None, parse_decl(b"var x, y, z []string"));

    assert_eq!(None, parse_decl(b"var x = a"));
    assert_eq!(None, parse_decl(b"var x, y = a, b"));
    assert_eq!(None, parse_decl(b"var x, y, z = a, b, c"));

    assert_eq!(None, parse_decl(b"var x int = a"));
    assert_eq!(None, parse_decl(b"var x, y int = a, b"));
    assert_eq!(None, parse_decl(b"var x, y, z int = a, b, c"));

    assert_eq!(None, parse_decl(b"var (x int; y float64;)"));
    assert_eq!(None, parse_decl(b"var (x, y int; z float64;)"));
    assert_eq!(None, parse_decl(b"var (x int; y, z float64;)"));

    assert_eq!(None, parse_decl(b"var (x = a; y = b;)"));
    assert_eq!(None, parse_decl(b"var (x, y = a, b; z = c;)"));
    assert_eq!(None, parse_decl(b"var (x = a; y, z = b, c;)"));

    assert_eq!(None, parse_decl(b"var (x int = a; y float64 = b;)"));
    assert_eq!(None, parse_decl(b"var (x, y string = a, b; z []int = c;)"));
    assert_eq!(None, parse_decl(b"var (x bool = a; y, z rune = b, c;)"));
}

#[test]
fn var_decl_err() {
    assert_eq!(Some(ET::InvalidVarDecl), parse_decl(b"var"));
    assert_eq!(Some(ET::InvalidVarDecl), parse_decl(b"var x"));
    assert_eq!(Some(ET::InvalidVarDecl), parse_decl(b"var ()"));
    assert_eq!(Some(ET::VarExprLengthMismatch), parse_decl(b"var x = a, b"));
    assert_eq!(Some(ET::VarExprLengthMismatch), parse_decl(b"var x, y = a"));
}
