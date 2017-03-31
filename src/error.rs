use std::error;
use std::fmt;
use std::result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GoreErrorType {
    Internal,

    // Scanner errors
    UnrecognizedCharacter,
    TrailingBlockComment,
    MalformedHexLiteral,
    TrailingString,
    TrailingRune,
    InvalidEscape,
    NewlineInString,
    NewlineInRune,
    EmptyRune,
}

impl GoreErrorType {
    fn to_str(&self) -> &str {
        use self::GoreErrorType::*;
        match *self {
            Internal => "internal compiler error",
            UnrecognizedCharacter => "unrecognized character",
            TrailingBlockComment => "unfinished block comment",
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

impl fmt::Display for GoreErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
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
    pub line: usize,
    pub col: usize
}

impl fmt::Display for GoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.col, self.ty)
    }
}

impl error::Error for GoreError {
    fn description(&self) -> &str {
        self.ty.description()
    }
}

impl GoreError {
    pub fn new(ty: GoreErrorType, line: usize,
               col: usize) -> GoreError {
        GoreError {
            ty: ty, line: line, col: col
        }
    }
}

pub type Result<T> = result::Result<T, GoreError>;

pub fn err<T>(ty: GoreErrorType, line: usize,
              col: usize) -> Result<T> {
    Err(GoreError::new(ty, line, col))
}
