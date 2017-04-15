extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::{parse_error, parse_decl};

#[test]
fn no_return_type() {
    assert!(parse_decl(b"func f() { }").is_ok());
}
