extern crate gore;

use gore::token::Token;
use gore::token::TokenType as TT;
use gore::error::GoreErrorType as ET;
use gore::error::Result;
use gore::scanner::Scanner;
use gore::parser::Parser;
use gore::untyped_ast as ast;

mod common;
use common::parse_error;


#[test]
fn test_package() {
    assert_eq!(Some(ET::MissingPackageDeclaration), parse_error(b""));
    assert_eq!(Some(ET::MissingPackageDeclaration), parse_error(b"main"));
    assert_eq!(Some(ET::MissingPackageName), parse_error(b"package"));
    assert_eq!(None, parse_error(b"package main"));
}
