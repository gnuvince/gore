use error::{Result, err};
use error::GoreErrorType as ET;
use token::Token;
use token::TokenType as TT;

#[derive(Debug)]
pub struct Scanner {
    filename: String,
    src: Vec<u8>,
    pos: usize,
    line: usize,
    col: usize
}

impl Scanner {
    pub fn new(filename: String, src: Vec<u8>) -> Scanner {
        Scanner {
            filename: filename,
            src: src,
            pos: 0,
            line: 1,
            col: 1
        }
    }

    fn looking_at(&self, bytes: &[u8]) -> bool {
        for (i, b) in bytes.iter().enumerate() {
            if self.peek_at(self.pos + i) != *b {
                return false;
            }
        }
        return true;
    }

    fn peek(&self) -> u8 {
        self.peek_at(self.pos)
    }

    fn peek_at(&self, offset: usize) -> u8 {
        if offset >= self.src.len() {
            return 0;
        }
        return self.src[offset];
    }

    fn advance(&mut self) {
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

    fn eat(&mut self, n: usize, ty: TT) -> Token {
        let t = Token { ty: ty, line: self.line, col: self.col, lexeme: None };
        for _ in 0 .. n {
            self.advance();
        }
        return t;
    }

    fn eof(&self) -> bool {
        self.peek() == 0
    }

    pub fn next(&mut self) -> Result<Token> {
        let () = self.skip_whitespace_and_comments()?;

        if self.eof() {
            return Ok(Token { ty: TT::Eof, line: self.line, col: self.col, lexeme: None });
        }

        // Operators and punctuation
        else if self.looking_at(b"<<=") { return Ok(self.eat(3, TT::LeftShiftEq)); }
        else if self.looking_at(b">>=") { return Ok(self.eat(3, TT::RightShiftEq)); }
        else if self.looking_at(b"+=")  { return Ok(self.eat(2, TT::PlusEq)); }
        else if self.looking_at(b"-=")  { return Ok(self.eat(2, TT::MinusEq)); }
        else if self.looking_at(b"*=")  { return Ok(self.eat(2, TT::StarEq)); }
        else if self.looking_at(b"/=")  { return Ok(self.eat(2, TT::SlashEq)); }
        else if self.looking_at(b"%=")  { return Ok(self.eat(2, TT::PercentEq)); }
        else if self.looking_at(b"|=")  { return Ok(self.eat(2, TT::BitorEq)); }
        else if self.looking_at(b"&=")  { return Ok(self.eat(2, TT::BitandEq)); }
        else if self.looking_at(b"++")  { return Ok(self.eat(2, TT::Incr)); }
        else if self.looking_at(b"--")  { return Ok(self.eat(2, TT::Decr)); }
        else if self.looking_at(b"<<")  { return Ok(self.eat(2, TT::LeftShift)); }
        else if self.looking_at(b">>")  { return Ok(self.eat(2, TT::RightShift)); }
        else if self.looking_at(b"&^")  { return Ok(self.eat(2, TT::BitClear)); }
        else if self.looking_at(b"&&")  { return Ok(self.eat(2, TT::And)); }
        else if self.looking_at(b"||")  { return Ok(self.eat(2, TT::Or)); }
        else if self.looking_at(b"==")  { return Ok(self.eat(2, TT::Eq)); }
        else if self.looking_at(b"!=")  { return Ok(self.eat(2, TT::Eq)); }
        else if self.looking_at(b"<=")  { return Ok(self.eat(2, TT::Le)); }
        else if self.looking_at(b">=")  { return Ok(self.eat(2, TT::Ge)); }
        else if self.looking_at(b"+")   { return Ok(self.eat(1, TT::Plus)); }
        else if self.looking_at(b"-")   { return Ok(self.eat(1, TT::Minus)); }
        else if self.looking_at(b"*")   { return Ok(self.eat(1, TT::Star)); }
        else if self.looking_at(b"/")   { return Ok(self.eat(1, TT::Slash)); }
        else if self.looking_at(b"%")   { return Ok(self.eat(1, TT::Percent)); }
        else if self.looking_at(b"^")   { return Ok(self.eat(1, TT::Bitnot)); }
        else if self.looking_at(b"&")   { return Ok(self.eat(1, TT::Bitand)); }
        else if self.looking_at(b"|")   { return Ok(self.eat(1, TT::Bitor)); }
        else if self.looking_at(b"!")   { return Ok(self.eat(1, TT::Not)); }
        else if self.looking_at(b"<")   { return Ok(self.eat(1, TT::Lt)); }
        else if self.looking_at(b">")   { return Ok(self.eat(1, TT::Gt)); }
        else if self.looking_at(b"(")   { return Ok(self.eat(1, TT::LParen)); }
        else if self.looking_at(b")")   { return Ok(self.eat(1, TT::RParen)); }
        else if self.looking_at(b"[")   { return Ok(self.eat(1, TT::LBracket)); }
        else if self.looking_at(b"]")   { return Ok(self.eat(1, TT::RBracket)); }
        else if self.looking_at(b"{")   { return Ok(self.eat(1, TT::LBrace)); }
        else if self.looking_at(b"}")   { return Ok(self.eat(1, TT::RBrace)); }
        else if self.looking_at(b",")   { return Ok(self.eat(1, TT::Comma)); }
        else if self.looking_at(b".")   { return Ok(self.eat(1, TT::Dot)); }
        else if self.looking_at(b";")   { return Ok(self.eat(1, TT::Semi)); }
        else if self.looking_at(b":")   { return Ok(self.eat(1, TT::Colon)); }

        return Err(err(ET::Internal, self.line, self.col));
    }

    // TODO(vfoley): ugly and nasty, refactor.
    fn skip_whitespace_and_comments(&mut self) -> Result<()> {
        loop {
            if is_whitespace(self.peek()) {
                self.skip_whitespace();
                continue;
            }

            if self.looking_at(b"//") {
                self.skip_line_comment();
                continue;
            }

            if self.looking_at(b"/*") {
                let () = self.skip_block_comment()?;
                continue;
            }

            break;
        }
        return Ok(());
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.peek()) {
            self.advance();
        }
    }

    fn skip_line_comment(&mut self) {
        // self.pos is still pointing at "//"
        self.advance();
        self.advance();
        while self.peek() != b'\n' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<()> {
        // self.pos is still pointing at "/*"
        let (line, col) = (self.line, self.col);
        self.advance();
        self.advance();
        while !self.eof() && !self.looking_at(b"*/") {
            self.advance();
        }
        if self.eof() {
            return Err(err(ET::TrailingBlockComment, line, col));
        } else {
            // skip over "*/"
            self.advance();
            self.advance();
            return Ok(());
        }
    }
}

fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n'
}


#[cfg(test)]
mod test {
    use ::token::Token;
    use ::token::TokenType as TT;
    use ::error::GoreErrorType as ET;

    fn assert_tok(expected_ty: TT, src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let tok_opt = scanner.next();
        let tok_ty = tok_opt.map(|tok| tok.ty).unwrap_or(TT::Eof);
        assert_eq!(tok_ty, expected_ty);
    }

    fn assert_id(name: &str, src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let tok_opt = scanner.next();
        match tok_opt {
            Ok(Token { ty: TT::Id, lexeme: Some(actual), .. }) => {
                assert_eq!(name, actual);
            }
            Ok(_) => { assert_eq!("assert_id", "didn't get an Id token"); }
            Err(_) => { assert_eq!("assert_id", "got an error"); }
        }
    }

    fn assert_err(expected_err: ET, src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let tok_opt = scanner.next();
        match tok_opt {
            Ok(_) => { assert_eq!("assert_eq", "got ok"); }
            Err(e) => { assert_eq!(expected_err, e.ty); }
        }
    }

    #[test]
    fn test_ops_and_punc() {
        assert_tok(TT::PlusEq, b"+=");
        assert_tok(TT::Incr, b"++");
        assert_tok(TT::Plus, b"+");
        assert_tok(TT::MinusEq, b"-=");
        assert_tok(TT::Decr, b"--");
        assert_tok(TT::Minus, b"-");
        assert_tok(TT::StarEq, b"*=");
        assert_tok(TT::Star, b"*");
        assert_tok(TT::SlashEq, b"/=");
        assert_tok(TT::Slash, b"/");
        assert_tok(TT::PercentEq, b"%=");
        assert_tok(TT::Percent, b"%");
        assert_tok(TT::Bitnot, b"^");
        assert_tok(TT::BitClear, b"&^");
        assert_tok(TT::BitandEq, b"&=");
        assert_tok(TT::Bitand, b"&");
        assert_tok(TT::BitorEq, b"|=");
        assert_tok(TT::Bitor, b"|");
        assert_tok(TT::LeftShiftEq, b"<<=");
        assert_tok(TT::LeftShift, b"<<");
        assert_tok(TT::RightShiftEq, b">>=");
        assert_tok(TT::RightShift, b">>");
        assert_tok(TT::And, b"&&");
        assert_tok(TT::Or, b"||");
        assert_tok(TT::Not, b"!");
        assert_tok(TT::Eq, b"==");
        assert_tok(TT::Eq, b"!=");
        assert_tok(TT::Le, b"<=");
        assert_tok(TT::Lt, b"<");
        assert_tok(TT::Ge, b">=");
        assert_tok(TT::Gt, b">");
        assert_tok(TT::LParen, b"(");
        assert_tok(TT::RParen, b")");
        assert_tok(TT::LBracket, b"[");
        assert_tok(TT::RBracket, b"]");
        assert_tok(TT::LBrace, b"{");
        assert_tok(TT::RBrace, b"}");
        assert_tok(TT::Comma, b",");
        assert_tok(TT::Dot, b".");
        assert_tok(TT::Semi, b";");
        assert_tok(TT::Colon, b":");
    }

    #[test]
    fn test_skip_whitespace_and_comments() {
        assert_tok(TT::Dot, b".");
        assert_tok(TT::Dot, b" .");
        assert_tok(TT::Dot, b"\t.");
        assert_tok(TT::Dot, b"\n.");
        assert_tok(TT::Dot, b"// comment\n.");
        assert_tok(TT::Dot, b"// comment 1\n  // comment 2\n\n.");
        assert_tok(TT::Dot, b"/* comment \n comment */   .");
        assert_err(ET::TrailingBlockComment, b"/* unfinished");
    }
}
