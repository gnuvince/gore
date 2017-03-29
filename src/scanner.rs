#[derive(Debug)]
pub struct Buffer {
    filename: String,
    src: Vec<u8>
    pos: usize,
    line: usize,
    col: usize
}

impl Buffer {
    pub fn new(filename: String, src: Vec<u8>) -> Buffer {
        Buffer {
            filename: filename,
            src: src,
            pos: 0,
            line: 1,
            col: 1
        }
    }

    pub fn peek(&self) -> u8 {
        self.peek_at(self.pos)
    }

    pub fn peek_next(&self) -> u8 {
        self.peek_at(self.pos + 1)
    }

    fn peek_at(&self, offset: usize) -> u8 {
        if offset >= self.src.len() {
            return 0;
        }
        return self.src[0];
    }

    pub fn advance(&mut self) {
        // unix-only newlines for now
        let is_newline = self.peek() == b'\n';
        self.pos += 1;
        if is_newline {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}
