use loc::Loc;
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
    col: usize,
    last_tok: TT
}

impl Scanner {
    // UTILITY METHODS

    pub fn new(filename: String, src: Vec<u8>) -> Scanner {
        Scanner {
            filename: filename,
            src: src,
            pos: 0,
            line: 1,
            col: 1,
            last_tok: TT::None
        }
    }

    /// Helper function to see if the next few characters match.
    fn looking_at(&self, bytes: &[u8]) -> bool {
        for (i, b) in bytes.iter().enumerate() {
            if self.peek_at(self.pos + i) != *b {
                return false;
            }
        }
        return true;
    }

    /// Return the character at the current position.
    fn peek(&self) -> u8 {
        self.peek_at(self.pos)
    }

    /// Return the character at the next position
    fn peek_next(&self) -> u8 {
        self.peek_at(self.pos + 1)
    }

    fn peek_at(&self, offset: usize) -> u8 {
        if offset >= self.src.len() {
            return 0;
        }
        return self.src[offset];
    }

    // TODO(vfoley): version with count?
    fn advance(&mut self) {
        // unix-only newlines for now
        if self.peek() == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += 1;
    }

    fn eat(&mut self, n: usize, ty: TT) -> Token {
        let t = Token::new(ty, self.loc(), None);
        for _ in 0 .. n {
            self.advance();
        }
        return t;
    }

    fn eof(&self) -> bool {
        self.peek() == 0
    }

    fn loc(&self) -> Loc {
        Loc::new(&self.filename, self.line, self.col)
    }


    // SCANNING METHODS

    pub fn next(&mut self) -> Result<Token> {
        // Skip whitespace and comments.
        // If a token is returned (i.e., a semi-colon
        // insertion), return that token.
        match self.skip_whitespace_and_comments() {
            Ok(Some(tok)) => {
                self.last_tok = tok.ty;
                return Ok(tok);
            }
            Ok(None) => { }
            Err(err) => { return Err(err); }
        }

        let tok = {
            // Insert a semi-colon, if necessary, in
            // a file that isn't newline-terminated.
            if self.eof() {
                if self.needs_semicolon() {
                    Token::new(TT::Semi, self.loc(), None)
                } else {
                    Token::new(TT::Eof, self.loc(), None)
                }
            }

            // Operators and punctuation.
            // OPTIMIZE(vfoley): use nesting to avoid
            // looking at the same character multiple times.
            else if self.looking_at(b"<<=") { self.eat(3, TT::LeftShiftEq) }
            else if self.looking_at(b">>=") { self.eat(3, TT::RightShiftEq) }
            else if self.looking_at(b"+=")  { self.eat(2, TT::PlusEq) }
            else if self.looking_at(b"-=")  { self.eat(2, TT::MinusEq) }
            else if self.looking_at(b"*=")  { self.eat(2, TT::StarEq) }
            else if self.looking_at(b"/=")  { self.eat(2, TT::SlashEq) }
            else if self.looking_at(b"%=")  { self.eat(2, TT::PercentEq) }
            else if self.looking_at(b"|=")  { self.eat(2, TT::BitorEq) }
            else if self.looking_at(b"&=")  { self.eat(2, TT::BitandEq) }
            else if self.looking_at(b"++")  { self.eat(2, TT::Incr) }
            else if self.looking_at(b"--")  { self.eat(2, TT::Decr) }
            else if self.looking_at(b"<<")  { self.eat(2, TT::LeftShift) }
            else if self.looking_at(b">>")  { self.eat(2, TT::RightShift) }
            else if self.looking_at(b"&^")  { self.eat(2, TT::BitClear) }
            else if self.looking_at(b"&&")  { self.eat(2, TT::And) }
            else if self.looking_at(b"||")  { self.eat(2, TT::Or) }
            else if self.looking_at(b"==")  { self.eat(2, TT::Eq) }
            else if self.looking_at(b"!=")  { self.eat(2, TT::Ne) }
            else if self.looking_at(b"<=")  { self.eat(2, TT::Le) }
            else if self.looking_at(b">=")  { self.eat(2, TT::Ge) }
            else if self.looking_at(b"=")   { self.eat(1, TT::Assign) }
            else if self.looking_at(b"+")   { self.eat(1, TT::Plus) }
            else if self.looking_at(b"-")   { self.eat(1, TT::Minus) }
            else if self.looking_at(b"*")   { self.eat(1, TT::Star) }
            else if self.looking_at(b"/")   { self.eat(1, TT::Slash) }
            else if self.looking_at(b"%")   { self.eat(1, TT::Percent) }
            else if self.looking_at(b"^")   { self.eat(1, TT::Bitnot) }
            else if self.looking_at(b"&")   { self.eat(1, TT::Bitand) }
            else if self.looking_at(b"|")   { self.eat(1, TT::Bitor) }
            else if self.looking_at(b"!")   { self.eat(1, TT::Not) }
            else if self.looking_at(b"<")   { self.eat(1, TT::Lt) }
            else if self.looking_at(b">")   { self.eat(1, TT::Gt) }
            else if self.looking_at(b"(")   { self.eat(1, TT::LParen) }
            else if self.looking_at(b")")   { self.eat(1, TT::RParen) }
            else if self.looking_at(b"[")   { self.eat(1, TT::LBracket) }
            else if self.looking_at(b"]")   { self.eat(1, TT::RBracket) }
            else if self.looking_at(b"{")   { self.eat(1, TT::LBrace) }
            else if self.looking_at(b"}")   { self.eat(1, TT::RBrace) }
            else if self.looking_at(b",")   { self.eat(1, TT::Comma) }
            else if self.looking_at(b";")   { self.eat(1, TT::Semi) }
            else if self.looking_at(b":")   { self.eat(1, TT::Colon) }
            // Special case: a float literal can start with a period in Go
            else if self.peek() == b'.' && is_digit(self.peek_next()) {
                self.number()?
            }
            else if self.looking_at(b".") { self.eat(1, TT::Dot) }
            else if is_alpha(self.peek()) { self.id_or_keyword() }
            else if is_digit(self.peek()) { self.number()? }
            else if self.peek() == b'"'   { self.interpreted_string()?}
            else if self.peek() == b'`'   { self.raw_string()? }
            else if self.peek() == b'\''  { self.rune()? }
            else {
                return err(ET::UnrecognizedCharacter, self.loc());
            }
        };
        self.last_tok = tok.ty;
        return Ok(tok);
    }

    // The return type can be one of three things:
    // - Ok(Some(Token{ ty: TT::Semi, .. })): when a semi-colon
    //   must be inserted in the token stream;
    // - Ok(None): when no token need be inserted in the stream;
    // - Err(error): when an error occurs (e.g., trailing block comment)
    // TODO(vfoley): ugly and nasty, refactor.
    fn skip_whitespace_and_comments(&mut self) -> Result<Option<Token>> {
        loop {
            if is_whitespace(self.peek()) {
                match self.skip_whitespace() {
                    None => { continue; }
                    some_tok => { return Ok(some_tok); }
                }
            }

            if self.looking_at(b"//") {
                self.skip_line_comment();
                continue;
            }

            if self.looking_at(b"/*") {
                match self.skip_block_comment() {
                    Ok(None) => { continue; }
                    Ok(some_tok) => { return Ok(some_tok); }
                    Err(err) => { return Err(err); }
                }
            }

            break;
        }
        return Ok(None);
    }

    fn needs_semicolon(&self) -> bool {
        match self.last_tok {
            TT::Id
            | TT::Blank
            | TT::Int
            | TT::IntHex
            | TT::Float
            | TT::Rune
            | TT::String
            | TT::Break
            | TT::Continue
            | TT::Return
            | TT::Incr
            | TT::Decr
            | TT::RParen
            | TT::RBracket
            | TT::RBrace
            | TT::Eof => true,
            _ => false
        }
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        while is_whitespace(self.peek()) {
            if self.peek() == b'\n' && self.needs_semicolon() {
                return Some(Token::new(TT::Semi, self.loc(), None));
            }
            self.advance();
        }
        return None;
    }

    fn skip_line_comment(&mut self) {
        // self.pos is still pointing at "//"
        self.advance();
        self.advance();
        while !self.eof() && self.peek() != b'\n' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<Option<Token>> {
        // self.pos is still pointing at "/*"
        let start_loc = self.loc();
        let mut has_newline = false;
        self.advance();
        self.advance();
        while !self.eof() && !self.looking_at(b"*/") {
            if self.peek() == b'\n' {
                has_newline = true;
            }
            self.advance();
        }
        if self.eof() {
            return err(ET::TrailingBlockComment, start_loc);
        } else {
            // skip over "*/"
            self.advance();
            self.advance();
            if has_newline && self.needs_semicolon() {
                return Ok(Some(Token::new(TT::Semi, start_loc, None)));
            } else {
                return Ok(None);
            }
        }
    }

    fn id_or_keyword(&mut self) -> Token {
        let start_loc = self.loc();
        let mut name = String::new();
        while is_alnum(self.peek()) {
            name.push(self.peek() as char);
            self.advance();
        }

        let (ty, lexeme) = {
            if name == "_" { (TT::Blank, None) }
            else if name == "break" { (TT::Break, None) }
            else if name == "case" { (TT::Case, None) }
            else if name == "continue" { (TT::Continue, None) }
            else if name == "default" { (TT::Default, None) }
            else if name == "else" { (TT::Else, None) }
            else if name == "for" { (TT::For, None) }
            else if name == "func" { (TT::Func, None) }
            else if name == "if" { (TT::If, None) }
            else if name == "package" { (TT::Package, None) }
            else if name == "return" { (TT::Return, None) }
            else if name == "struct" { (TT::Struct, None) }
            else if name == "switch" { (TT::Switch, None) }
            else if name == "type" { (TT::Type, None) }
            else if name == "var" { (TT::Var, None) }
            else if name == "append" { (TT::Append, None) }
            else if name == "print" { (TT::Print, None) }
            else if name == "println" { (TT::Println, None) }
            else { (TT::Id, Some(name)) }
        };
        return Token::new(ty, start_loc, lexeme);
    }

    fn number(&mut self) -> Result<Token> {
        if self.looking_at(b"0x") || self.looking_at(b"0X") {
            return self.hex();
        } else {
            return self.decimal_or_octal();
        }
    }

    fn hex(&mut self) -> Result<Token> {
        let start_loc = self.loc();
        let mut digits = String::new();
        // skip over "0x" or "0X"
        self.advance();
        self.advance();
        while is_hex(self.peek()) {
            digits.push(self.peek() as char);
            self.advance();
        }
        if digits.is_empty() {
            return err(ET::MalformedHexLiteral, start_loc);
        } else {
            return Ok(Token::new(TT::IntHex, start_loc, Some(digits)));
        }
    }

    fn decimal_or_octal(&mut self) -> Result<Token> {
        let start_loc = self.loc();
        let mut digits = String::new();

        while is_digit(self.peek()) {
            digits.push(self.peek() as char);
            self.advance();
        }

        if self.peek() == b'.' {
            digits.push('.');
            self.advance();
            return self.float_literal(start_loc, digits);
        } else {
            return Ok(Token::new(TT::Int, start_loc, Some(digits)));
        }
    }

    fn float_literal(&mut self, loc: Loc, mut digits: String) ->
        Result<Token> {
            while is_digit(self.peek()) {
                digits.push(self.peek() as char);
                self.advance();
            }
            return Ok(Token::new(TT::Float, loc, Some(digits)));
    }

    fn interpreted_string(&mut self) -> Result<Token> {
        let start_loc = self.loc();
        let mut content = String::new();

        self.advance(); // consume opening double-quote
        while self.peek() != b'"' {
            if self.eof() {
                return err(ET::TrailingString, start_loc);
            }

            if self.peek() == b'\\' {
                self.advance();
                let code: u8 = match self.peek() {
                    b'a' => { 0x07 }
                    b'b' => { 0x08 }
                    b'f' => { 0x0c }
                    b'n' => { 0x0a }
                    b'r' => { 0x0d }
                    b't' => { 0x09 }
                    b'v' => { 0x0b }
                    b'\\' => { 0x5c }
                    b'"' => { 0x22 }
                    _ => { return err(ET::InvalidEscape, self.loc()); }
                };
                content.push(code as char);
                self.advance();
            } else if self.peek() == b'\n' {
                return err(ET::NewlineInString, self.loc());
            } else {
                content.push(self.peek() as char);
                self.advance();
            }
        }
        self.advance();

        return Ok(Token::new(TT::String, start_loc, Some(content)));
    }

    fn raw_string(&mut self) -> Result<Token> {
        let start_loc = self.loc();
        let mut content = String::new();

        self.advance(); // consume opening back-quote
        while self.peek() != b'`' {
            if self.eof() {
                return err(ET::TrailingString, start_loc);
            }
            // Carriage returns are discarded in raw strings
            if self.looking_at(b"\\r") {
                self.advance();
                self.advance();
            } else {
                content.push(self.peek() as char);
                self.advance();
            }
        }
        self.advance();

        return Ok(Token::new(TT::String, start_loc, Some(content)));
    }

    fn rune(&mut self) -> Result<Token> {
        let start_loc = self.loc();
        let mut content = String::new();

        self.advance(); // opening single-quote
        if self.peek() == b'\\' {
            self.advance();
            let code: u8 = match self.peek() {
                b'a' => { 0x07 }
                b'b' => { 0x08 }
                b'f' => { 0x0c }
                b'n' => { 0x0a }
                b'r' => { 0x0d }
                b't' => { 0x09 }
                b'v' => { 0x0b }
                b'\\' => { 0x5c }
                b'\'' => { 0x27 }
                _ => { return err(ET::InvalidEscape, self.loc()); }
            };
            content.push(code as char);
            self.advance();
        } else if self.peek() == b'\n' {
            return err(ET::NewlineInRune, self.loc());
        } else if self.peek() == b'\'' {
            return err(ET::EmptyRune, start_loc);
        } else {
            content.push(self.peek() as char);
            self.advance();
        }

        if self.peek() == b'\'' {
            self.advance();
            return Ok(Token::new(TT::Rune, start_loc, Some(content)));
        } else {
            return err(ET::TrailingRune, start_loc);
        }
    }
}

fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n'
}

fn is_alpha(b: u8) -> bool {
    b == b'_' || (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z')
}

fn is_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

fn is_hex(b: u8) -> bool {
    is_digit(b) || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}

fn is_alnum(b: u8) -> bool {
    is_alpha(b) || is_digit(b)
}


/* These tests are not in tests/scanner_tests.rs because
 * the access a private field of the Scanner struct.
 */
#[cfg(test)]
mod test {
    use ::scanner::Scanner;
    use ::token::TokenType as TT;
    #[test]
    fn test_last_tok() {
        let input = b"x += -4".to_vec();
        let mut s = Scanner::new("-".to_string(), input);
        assert_eq!(TT::None, s.last_tok);

        assert!(s.next().is_ok());
        assert_eq!(TT::Id, s.last_tok);

        assert!(s.next().is_ok());
        assert_eq!(TT::PlusEq, s.last_tok);

        assert!(s.next().is_ok());
        assert_eq!(TT::Minus, s.last_tok);

        assert!(s.next().is_ok());
        assert_eq!(TT::Int, s.last_tok);
    }
}
