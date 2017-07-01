use loc::Loc;
use error::{Result, err};
use error::ErrorType as ET;
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

    /// Returns `true` if the next elements of the
    /// input buffer (`src`) are equal to `bytes`.
    fn looking_at(&mut self, bytes: &[u8]) -> bool {
        for (i, b) in bytes.iter().enumerate() {
            if self.peek_at(self.pos + i) != *b {
                return false;
            }
        }
        self.pos += bytes.len();
        return true;
    }

    /// Returns the byte at the current position.
    fn peek(&self) -> u8 {
        self.peek_at(self.pos)
    }

    /// Returns the byte at the next position.
    fn peek_next(&self) -> u8 {
        self.peek_at(self.pos + 1)
    }

    /// Returns the byte at the current position,
    /// or 0 if at the end of the input.
    fn peek_at(&self, offset: usize) -> u8 {
        if offset >= self.src.len() {
            return 0;
        }
        return self.src[offset];
    }

    /// Moves the current position forward by one
    /// and updates the line and column information.
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

    /// Consumes `n` bytes of input and returns
    /// a token of the given type starting positioned
    /// at the current location.
    fn tok_nolex(&mut self, ty: TT) -> Token {
        let t = Token::new(ty, self.loc(), None);
        return t;
    }

    /// Returns true if the input has been entirely consumed.
    fn eof(&self) -> bool {
        self.pos >= self.src.len()
    }


    /// Returns the current location in the input.
    fn loc(&self) -> Loc {
        Loc::new(&self.filename, self.line, self.col)
    }


    // SCANNING METHODS

    /// Consumes the input and returns either the
    /// next token or an error.
    ///
    /// This function should be called in a loop
    /// until a token with the type `TokenType::EOF`
    /// is returned.
    // TODO(vfoley): make scanner into an iterator?
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
            else if self.looking_at(b"<<=") { self.tok_nolex(TT::LeftShiftEq) }
            else if self.looking_at(b">>=") { self.tok_nolex(TT::RightShiftEq) }
            else if self.looking_at(b":=")  { self.tok_nolex(TT::ColonEq) }
            else if self.looking_at(b"+=")  { self.tok_nolex(TT::PlusEq) }
            else if self.looking_at(b"-=")  { self.tok_nolex(TT::MinusEq) }
            else if self.looking_at(b"*=")  { self.tok_nolex(TT::StarEq) }
            else if self.looking_at(b"/=")  { self.tok_nolex(TT::SlashEq) }
            else if self.looking_at(b"%=")  { self.tok_nolex(TT::PercentEq) }
            else if self.looking_at(b"|=")  { self.tok_nolex(TT::BitorEq) }
            else if self.looking_at(b"&=")  { self.tok_nolex(TT::BitandEq) }
            else if self.looking_at(b"++")  { self.tok_nolex(TT::Incr) }
            else if self.looking_at(b"--")  { self.tok_nolex(TT::Decr) }
            else if self.looking_at(b"<<")  { self.tok_nolex(TT::LeftShift) }
            else if self.looking_at(b">>")  { self.tok_nolex(TT::RightShift) }
            else if self.looking_at(b"&^")  { self.tok_nolex(TT::BitClear) }
            else if self.looking_at(b"&&")  { self.tok_nolex(TT::And) }
            else if self.looking_at(b"||")  { self.tok_nolex(TT::Or) }
            else if self.looking_at(b"==")  { self.tok_nolex(TT::Eq) }
            else if self.looking_at(b"!=")  { self.tok_nolex(TT::Ne) }
            else if self.looking_at(b"<=")  { self.tok_nolex(TT::Le) }
            else if self.looking_at(b">=")  { self.tok_nolex(TT::Ge) }
            else if self.looking_at(b"=")   { self.tok_nolex(TT::Assign) }
            else if self.looking_at(b"+")   { self.tok_nolex(TT::Plus) }
            else if self.looking_at(b"-")   { self.tok_nolex(TT::Minus) }
            else if self.looking_at(b"*")   { self.tok_nolex(TT::Star) }
            else if self.looking_at(b"/")   { self.tok_nolex(TT::Slash) }
            else if self.looking_at(b"%")   { self.tok_nolex(TT::Percent) }
            else if self.looking_at(b"^")   { self.tok_nolex(TT::Bitnot) }
            else if self.looking_at(b"&")   { self.tok_nolex(TT::Bitand) }
            else if self.looking_at(b"|")   { self.tok_nolex(TT::Bitor) }
            else if self.looking_at(b"!")   { self.tok_nolex(TT::Not) }
            else if self.looking_at(b"<")   { self.tok_nolex(TT::Lt) }
            else if self.looking_at(b">")   { self.tok_nolex(TT::Gt) }
            else if self.looking_at(b"(")   { self.tok_nolex(TT::LParen) }
            else if self.looking_at(b")")   { self.tok_nolex(TT::RParen) }
            else if self.looking_at(b"[")   { self.tok_nolex(TT::LBracket) }
            else if self.looking_at(b"]")   { self.tok_nolex(TT::RBracket) }
            else if self.looking_at(b"{")   { self.tok_nolex(TT::LBrace) }
            else if self.looking_at(b"}")   { self.tok_nolex(TT::RBrace) }
            else if self.looking_at(b",")   { self.tok_nolex(TT::Comma) }
            else if self.looking_at(b";")   { self.tok_nolex(TT::Semi) }
            else if self.looking_at(b":")   { self.tok_nolex(TT::Colon) }
            // Special case: a float literal can start with a period in Go
            else if self.peek() == b'.' && is_digit(self.peek_next()) {
                self.number()?
            }
            else if self.looking_at(b".") { self.tok_nolex(TT::Dot) }
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
    // TODO(vfoley): ugly and nasty: refactor.
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
                let has_newline = self.skip_block_comment()?;
                if has_newline && self.needs_semicolon() {
                    let start_loc = self.loc();
                    return Ok(Some(Token::new(TT::Semi, start_loc, None)));
                } else {
                    continue;
                }
            }

            break;
        }
        return Ok(None);
    }

    /// Returns `true` if a token at the end of
    /// a line needs to be followed by a semi-colon.
    /// Ref.: https://golang.org/ref/spec#Semicolons
    fn needs_semicolon(&self) -> bool {
        match self.last_tok {
            TT::Id
            | TT::Blank
            | TT::Int
            | TT::IntOct
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

    /// Returns `Ok(Semicolon)` if a newline occurs after
    /// a token that terminates a statement, otherwise returns
    /// `None`.
    fn skip_whitespace(&mut self) -> Option<Token> {
        while is_whitespace(self.peek()) {
            if self.peek() == b'\n' && self.needs_semicolon() {
                return Some(Token::new(TT::Semi, self.loc(), None));
            }
            self.advance();
        }
        return None;
    }

    /// Skips over a line comment, but does not consume
    /// the newline character at the end.
    fn skip_line_comment(&mut self) {
        while !self.eof() && self.peek() != b'\n' {
            self.advance();
        }
    }

    /// Skips over a block comment.
    /// Returns `Some(true)` if the comment contains at least one newline;
    /// returns `Some(false)` if the comment doesn't contain a newline;
    /// return `Err(...)` otherwise.
    fn skip_block_comment(&mut self) -> Result<bool> {
        let start_loc = self.loc();
        let mut has_newline = false;
        while !self.eof() && !self.looking_at(b"*/") {
            has_newline = has_newline || self.peek() == b'\n';
            self.advance();
        }
        if self.eof() {
            return err(ET::TrailingBlockComment, start_loc);
        } else {
            return Ok(has_newline);
        }
    }

    /// Returns a token for an identifier or a keyword.
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

    /// Returns a token for a numeric literal.
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
        let starts_with_zero = self.peek() == b'0';
        let mut digits = String::new();

        while is_digit(self.peek()) {
            digits.push(self.peek() as char);
            self.advance();
        }

        if self.peek() == b'.' {
            digits.push('.');
            self.advance();
            return self.float_literal(start_loc, digits);
        } else if starts_with_zero {
            if digits.bytes().all(|d| d >= b'0' && d <= b'7') {
                return Ok(Token::new(TT::IntOct, start_loc, Some(digits)));
            } else {
                return err(ET::MalformedOctLiteral, start_loc);
            }
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
                continue;
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
