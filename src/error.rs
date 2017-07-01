use loc::Loc;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub fn err<T>(ty: ErrorType, loc: Loc) -> Result<T> {
    Err(Error::new(ty, loc))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorType {
    Internal,

    // Scanner errors
    UnrecognizedCharacter,
    TrailingBlockComment,
    MalformedOctLiteral,
    MalformedHexLiteral,
    TrailingString,
    TrailingRune,
    InvalidEscape,
    NewlineInString,
    NewlineInRune,
    EmptyRune,
}

impl ErrorType {
    fn to_str(&self) -> &str {
        use self::ErrorType::*;
        match *self {
            Internal => "internal compiler error",
            UnrecognizedCharacter => "unrecognized character",
            TrailingBlockComment => "unfinished block comment",
            MalformedOctLiteral => "malformed octal literal",
            MalformedHexLiteral => "malformed hexadecimal literal",
            TrailingString => "unfinished string literal",
            TrailingRune => "unfinished rune literal",
            InvalidEscape => "invalid escape code",
            NewlineInString => "newline in interpreted string literal",
            NewlineInRune => "newline in rune literal",
            EmptyRune => "empty rune literal",
        }
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug)]
pub struct Error {
    pub ty: ErrorType,
    pub loc: Loc
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.loc, self.ty)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.ty.to_str()
    }
}

impl Error {
    pub fn new(ty: ErrorType, loc: Loc) -> Error {
        Error { ty: ty, loc: loc }
    }
}
