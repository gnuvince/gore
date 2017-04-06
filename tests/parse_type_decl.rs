extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn type_decl_ok() {
    assert_eq!(None, parse_decl(b"type num int"));
    assert_eq!(None, parse_decl(b"type reals []float64"));
    assert_eq!(None, parse_decl(b"type (re float64; im float64;)"));
    assert_eq!(None, parse_decl(b"type (
re float64
im float64
)"));
}

#[test]
fn type_decl_error() {
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type"));
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type real"));
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type ()"));
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type (num)"));
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type (num int; real)"));
    assert_eq!(Some(ET::InvalidTypeDecl), parse_decl(b"type (42)"));
}
