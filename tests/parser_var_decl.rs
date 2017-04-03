extern crate gore;

use gore::error::GoreErrorType as ET;

mod common;
use common::parse_error;

#[test]
fn test_var_decl() {
    assert_eq!(None, parse_error(VAR_DECL_OK_01));
    assert_eq!(None, parse_error(VAR_DECL_OK_02));
    assert_eq!(None, parse_error(VAR_DECL_OK_03));
    assert_eq!(None, parse_error(VAR_DECL_OK_04));

    assert_eq!(Some(ET::UnexpectedToken), parse_error(VAR_DECL_ERR_01));
    assert_eq!(Some(ET::InvalidVarDecl), parse_error(VAR_DECL_ERR_02));
}

const VAR_DECL_OK_01: &'static [u8] = b"
package main
var x int
";

const VAR_DECL_OK_02: &'static [u8] = b"
package main
var x = y
";

const VAR_DECL_OK_03: &'static [u8] = b"
package main
var x []float64 = y
";

const VAR_DECL_OK_04: &'static [u8] = b"
package main
var x int, y float64
";

const VAR_DECL_ERR_01: &'static [u8] = b"
package main
var
";

const VAR_DECL_ERR_02: &'static [u8] = b"
package main
var x
";
