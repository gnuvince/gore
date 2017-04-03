use loc::Loc;
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

    fn code(&self) -> (char, u16) {
        use self::GoreErrorType::*;
        match *self {
            Internal => ('I', 1),
            UnrecognizedCharacter => ('S', 1),
            TrailingBlockComment => ('S', 2),
            MalformedHexLiteral => ('S', 3),
            TrailingString => ('S', 4),
            TrailingRune => ('S', 5),
            InvalidEscape => ('S', 6),
            NewlineInString => ('S', 7),
            NewlineInRune => ('S', 8),
            EmptyRune => ('S', 9),
        }
    }
}

impl fmt::Display for GoreErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (category, id) = self.code();
        write!(f, "{} ({}{:03})", self.to_str(), category, id)
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
