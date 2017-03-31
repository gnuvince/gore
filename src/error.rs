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

#[derive(Debug)]
pub struct GoreError {
    pub ty: GoreErrorType,
    pub line: usize,
    pub col: usize
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
