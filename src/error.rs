use loc::Loc;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GoreErrorType {
    Internal,

    // Scanner errors
    EmptyRune,
    InvalidEscape,
    MalformedHexLiteral,
    NewlineInRune,
    NewlineInString,
    TrailingBlockComment,
    TrailingRune,
    TrailingString,
    UnrecognizedCharacter,

    // Parser errors
    ExpectedDeclaration,
    ExpectedExpression,
    InvalidVarDecl,
    MissingLexeme,
    MissingPackageDeclaration,
    MissingPackageName,
    MissingVariableName,
    UnexpectedToken,
    VarExprLengthMismatch,
    InvalidTypeDecl,
}

impl GoreErrorType {
    fn to_str(&self) -> &str {
        use self::GoreErrorType::*;
        match *self {
            Internal => "internal compiler error",

            EmptyRune => "empty rune literal",
            InvalidEscape => "invalid escape code",
            MalformedHexLiteral => "malformed hexadecimal literal",
            NewlineInRune => "newline in rune literal",
            NewlineInString => "newline in interpreted string literal",
            TrailingBlockComment => "unfinished block comment",
            TrailingRune => "unfinished rune literal",
            TrailingString => "unfinished string literal",
            UnrecognizedCharacter => "unrecognized character",

            ExpectedDeclaration => "expected declaration",
            ExpectedExpression => "expected expression",
            InvalidVarDecl => "invalid var declaration",
            MissingLexeme => "lexeme is missing",
            MissingPackageDeclaration => "missing package declaration",
            MissingPackageName => "package name is missing",
            MissingVariableName => "missing variable name",
            UnexpectedToken => "unexpected token",
            VarExprLengthMismatch =>
                "variable list and expression list must have the same length",
            InvalidTypeDecl => "invalid type declaration"
        }
    }
}

impl fmt::Display for GoreErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (E{:03})", self.to_str(), *self as u16)
    }
}

impl error::Error for GoreErrorType {
    fn description(&self) -> &str {
        self.to_str()
    }
}


#[derive(Debug)]
pub struct GoreError {
    pub ty: GoreErrorType,
    pub loc: Loc
}

impl fmt::Display for GoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.loc, self.ty)
    }
}

impl error::Error for GoreError {
    fn description(&self) -> &str {
        self.ty.description()
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
