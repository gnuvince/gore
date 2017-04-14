use std::error::{self, Error};
use std::fmt;
use std::result;

use loc::Loc;
use token::TokenType as TT;

#[derive(Debug, Clone)]
pub enum GoreErrorType {
    Internal(String),

    // Scanner errors
    EmptyRune,
    InvalidEscape(char),
    EmptyHexLiteral,
    NewlineInRune,
    NewlineInString,
    TrailingBlockComment,
    TrailingRune,
    TrailingString,
    UnrecognizedCharacter(char),

    // Parser errors
    ExpectedDeclaration,
    ExpectedExpression,
    InvalidVarDecl,
    MissingLexeme,
    MissingPackageDeclaration,
    MissingPackageName,
    MissingVariableName,
    UnexpectedToken(TT, Vec<TT>),
    VarExprLengthMismatch,
    InvalidTypeDecl,
    ExpectedParamList,
}

impl GoreErrorType {
    fn extra_info(&self) -> String {
        use self::GoreErrorType::*;
        match *self {
            Internal(ref s) => s.to_string(),
            InvalidEscape(c) => format!("'\\{}'", c),
            UnrecognizedCharacter(c) => format!("{}", c),
            UnexpectedToken(actual, ref expected) =>
                format!("found: {}; expected: {:?}", actual, expected),
            _ => String::new()
        }
    }
}


#[derive(Debug)]
pub struct GoreError {
    pub ty: GoreErrorType,
    pub loc: Loc
}

impl fmt::Display for GoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.loc, self.description())?;
        let extra_info = self.ty.extra_info();
        if !extra_info.is_empty() {
            write!(f, ": {}", extra_info)?;
        }
        Ok(())
    }
}

impl error::Error for GoreError {
    fn description(&self) -> &str {
        use self::GoreErrorType::*;
        match self.ty {
            Internal(_) => "internal compiler error",

            EmptyRune => "empty rune literal",
            InvalidEscape(_) => "invalid escape code",
            EmptyHexLiteral => "empty hexadecimal literal",
            NewlineInRune => "newline in rune literal",
            NewlineInString => "newline in interpreted string literal",
            TrailingBlockComment => "unfinished block comment",
            TrailingRune => "unfinished rune literal",
            TrailingString => "unfinished string literal",
            UnrecognizedCharacter(_) => "unrecognized character",

            ExpectedDeclaration => "expected declaration (var, type, func)",
            ExpectedExpression => "expected expression",
            InvalidVarDecl => "invalid var declaration",
            MissingLexeme => "lexeme is missing",
            MissingPackageDeclaration => "missing package declaration",
            MissingPackageName => "package name is missing",
            MissingVariableName => "missing variable name",
            UnexpectedToken(_, _) => "unexpected token",
            VarExprLengthMismatch =>
                "variable list and expression list must have the same length",
            InvalidTypeDecl => "invalid type declaration",
            ExpectedParamList => "expected list of parameters",
        }
    }
}

impl GoreError {
    pub fn new(ty: GoreErrorType, loc: Loc) -> GoreError {
        GoreError {
            ty: ty, loc: loc
        }
    }
}

pub type Result<T> = result::Result<T, GoreError>;

pub fn err<T>(ty: GoreErrorType, loc: Loc) -> Result<T> {
    Err(GoreError::new(ty, loc))
}
