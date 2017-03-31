use std::fmt;

#[derive(Debug, Clone)]
pub struct Loc {
    pub filename: String,
    pub line: usize,
    pub col: usize
}


impl Loc {
    pub fn new(filename: &str, line: usize, col: usize) -> Loc {
        Loc { filename: filename.to_string(), line: line, col: col }
    }
}


impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.col)
    }
}
