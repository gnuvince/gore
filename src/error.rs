use std::result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GoreErrorType {
    Internal,

    // Scanner errors
    UnrecognizedCharacter,
    TrailingBlockComment,
    MalformedHexLiteral
}

#[derive(Debug)]
pub struct GoreError {
    pub ty: GoreErrorType,
    pub line: usize,
    pub col: usize
}

pub type Result<T> = result::Result<T, GoreError>;

pub fn err(ty: GoreErrorType, line: usize, col: usize) -> GoreError {
    GoreError { ty: ty, line: line, col: col }
}
