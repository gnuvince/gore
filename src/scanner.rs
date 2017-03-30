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

    fn peek_next(&self) -> u8 {
        self.peek_at(self.pos + 1)
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

    fn pos(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn next(&mut self) -> Result<Token> {
        match self.skip_whitespace_and_comments() {
            Ok(Some(tok)) => {
                self.last_tok = tok.ty;
                return Ok(tok);
            }
            Ok(None) => { }
            Err(err) => { return Err(err); }
        }

        let tok = {
            if self.eof() {
                if self.needs_semicolon() {
                    Token {
                        ty: TT::Semi,
                        line: self.line,
                        col: self.col,
                        lexeme: None
                    }
                } else {
                    Token {
                        ty: TT::Eof,
                        line: self.line,
                        col: self.col,
                        lexeme: None
                    }
                }
            }

            // Operators and punctuation
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
            else if self.looking_at(b".") {
                self.eat(1, TT::Dot)
            }

            else if is_alpha(self.peek()) {
                self.id_or_keyword()
            }

            else if is_digit(self.peek()) {
                self.number()?
            }
            else {
                return Err(err(ET::UnrecognizedCharacter, self.line, self.col));
            }
        };
        self.last_tok = tok.ty;
        return Ok(tok);
    }

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
                let () = self.skip_block_comment()?;
                continue;
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
                return Some(Token {
                    ty: TT::Semi,
                    line: self.line,
                    col: self.col,
                    lexeme: None
                });
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

    fn skip_block_comment(&mut self) -> Result<()> {
        // self.pos is still pointing at "/*"
        let (line, col) = self.pos();
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

    fn id_or_keyword(&mut self) -> Token {
        let (line, col) = self.pos();
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
        return Token {ty: ty, line: line, col: col, lexeme: lexeme};
    }

    fn number(&mut self) -> Result<Token> {
        if self.looking_at(b"0x") || self.looking_at(b"0X") {
            return self.hex();
        } else {
            return self.decimal_or_octal();
        }
    }

    fn hex(&mut self) -> Result<Token> {
        let (line, col) = self.pos();
        let mut digits = String::new();
        // skip over "0x" or "0X"
        self.advance();
        self.advance();
        while is_hex(self.peek()) {
            digits.push(self.peek() as char);
            self.advance();
        }
        if digits.is_empty() {
            return Err(err(ET::MalformedHexLiteral, line, col));
        } else {
            return Ok(Token {
                ty: TT::IntHex,
                line: line,
                col: col,
                lexeme: Some(digits)
            });
        }
    }

    fn decimal_or_octal(&mut self) -> Result<Token> {
        let (line, col) = self.pos();
        let mut digits = String::new();

        while is_digit(self.peek()) {
            digits.push(self.peek() as char);
            self.advance();
        }

        if self.peek() == b'.' {
            digits.push('.');
            self.advance();
            return self.float_literal(line, col, digits);
        } else {
            return Ok(Token {
                ty: TT::Int,
                line: line,
                col: col,
                lexeme: Some(digits)
            });
        }
    }

    fn float_literal(&mut self, line: usize, col: usize, mut digits: String) ->
        Result<Token> {
            while is_digit(self.peek()) {
                digits.push(self.peek() as char);
                self.advance();
            }
            return Ok(Token {
                ty: TT::Float,
                line: line,
                col: col,
                lexeme: Some(digits)
            });
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


#[cfg(test)]
mod test {
    use ::token::Token;
    use ::token::TokenType as TT;
    use ::error::GoreErrorType as ET;
    use super::Scanner;

    fn assert_tok(expected_ty: TT, src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let tok_opt = scanner.next();
        let tok_ty = tok_opt.map(|tok| tok.ty).unwrap_or(TT::Eof);
        assert_eq!(tok_ty, expected_ty);
    }

    fn assert_toks(expected_tys: &[TT], src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let mut toks = Vec::new();
        loop {
            match scanner.next() {
                Ok(tok) => {
                    if tok.ty == TT::Eof {
                        break;
                    }
                    toks.push(tok.ty);
                }
                Err(_) => { assert_eq!("assert_toks", "got an error"); }
            }
        }
        assert!(
            expected_tys.len() == toks.len() &&
                expected_tys.iter().zip(&toks).all(|(a, b)| *a == *b)
        );
    }

    fn assert_lexeme(expected: &str, src: &[u8]) {
        let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
        let mut scanner = super::Scanner::new("-".to_string(), src_vec);
        let tok_opt = scanner.next();
        match tok_opt {
            Ok(Token { lexeme: Some(actual), .. }) => {
                assert_eq!(expected, actual);
            }
            Ok(_) => { assert_eq!("assert_id", "got token with no lexeme"); }
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
    fn test_invalid_characters() {
        assert_err(ET::UnrecognizedCharacter, b"#");
        assert_err(ET::UnrecognizedCharacter, b"$");
        assert_err(ET::UnrecognizedCharacter, b"?");
        assert_err(ET::UnrecognizedCharacter, b"~");
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
        assert_tok(TT::Ne, b"!=");
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
        assert_tok(TT::Eof, b"// only a line comment\n");
        assert_tok(TT::Eof, b"// only a line comment");
        assert_err(ET::TrailingBlockComment, b"/* unfinished");
    }

    #[test]
    fn test_keywords() {
        assert_tok(TT::Break, b"break");
        assert_tok(TT::Case, b"case");
        assert_tok(TT::Continue, b"continue");
        assert_tok(TT::Default, b"default");
        assert_tok(TT::Else, b"else");
        assert_tok(TT::For, b"for");
        assert_tok(TT::Func, b"func");
        assert_tok(TT::If, b"if");
        assert_tok(TT::Package, b"package");
        assert_tok(TT::Return, b"return");
        assert_tok(TT::Struct, b"struct");
        assert_tok(TT::Switch, b"switch");
        assert_tok(TT::Type, b"type");
        assert_tok(TT::Var, b"var");
        assert_tok(TT::Append, b"append");
        assert_tok(TT::Print, b"print");
        assert_tok(TT::Println, b"println");
    }

    #[test]
    fn test_ids() {
        assert_tok(TT::Id, b"foo");
        assert_tok(TT::Id, b"Foo");
        assert_tok(TT::Id, b"_foo");
        assert_tok(TT::Id, b"_1");
        assert_tok(TT::Id, b"__LINE__");
        assert_tok(TT::Blank, b"_");

        assert_lexeme("foo", b"foo");
        assert_lexeme("Foo", b"Foo");
        assert_lexeme("_foo", b"_foo");
        assert_lexeme("_1", b"_1");
        assert_lexeme("__LINE__", b"__LINE__");
    }

    #[test]
    fn test_hex() {
        assert_tok(TT::IntHex, b"0x0");
        assert_tok(TT::IntHex, b"0x1");
        assert_tok(TT::IntHex, b"0x2");
        assert_tok(TT::IntHex, b"0x3");
        assert_tok(TT::IntHex, b"0x4");
        assert_tok(TT::IntHex, b"0x5");
        assert_tok(TT::IntHex, b"0x6");
        assert_tok(TT::IntHex, b"0x7");
        assert_tok(TT::IntHex, b"0x8");
        assert_tok(TT::IntHex, b"0x9");
        assert_tok(TT::IntHex, b"0xa");
        assert_tok(TT::IntHex, b"0xb");
        assert_tok(TT::IntHex, b"0xc");
        assert_tok(TT::IntHex, b"0xd");
        assert_tok(TT::IntHex, b"0xe");
        assert_tok(TT::IntHex, b"0xf");
        assert_tok(TT::IntHex, b"0XA");
        assert_tok(TT::IntHex, b"0XB");
        assert_tok(TT::IntHex, b"0XC");
        assert_tok(TT::IntHex, b"0XD");
        assert_tok(TT::IntHex, b"0XE");
        assert_tok(TT::IntHex, b"0XF");

        assert_lexeme("0", b"0x0");
        assert_lexeme("1", b"0x1");
        assert_lexeme("2", b"0x2");
        assert_lexeme("3", b"0x3");
        assert_lexeme("4", b"0x4");
        assert_lexeme("5", b"0x5");
        assert_lexeme("6", b"0x6");
        assert_lexeme("7", b"0x7");
        assert_lexeme("8", b"0x8");
        assert_lexeme("9", b"0x9");
        assert_lexeme("a", b"0xa");
        assert_lexeme("b", b"0xb");
        assert_lexeme("c", b"0xc");
        assert_lexeme("d", b"0xd");
        assert_lexeme("e", b"0xe");
        assert_lexeme("f", b"0xf");
        assert_lexeme("A", b"0XA");
        assert_lexeme("B", b"0XB");
        assert_lexeme("C", b"0XC");
        assert_lexeme("D", b"0XD");
        assert_lexeme("E", b"0XE");
        assert_lexeme("F", b"0XF");

        assert_err(ET::MalformedHexLiteral, b"0x");
        assert_err(ET::MalformedHexLiteral, b"0X");
    }

    #[test]
    fn test_octal() {
        assert_tok(TT::Int, b"0");
        assert_tok(TT::Int, b"01");
        assert_tok(TT::Int, b"02");
        assert_tok(TT::Int, b"03");
        assert_tok(TT::Int, b"04");
        assert_tok(TT::Int, b"05");
        assert_tok(TT::Int, b"06");
        assert_tok(TT::Int, b"07");
        assert_tok(TT::Int, b"0377");

        assert_lexeme("0", b"0");
        assert_lexeme("01", b"01");
        assert_lexeme("02", b"02");
        assert_lexeme("03", b"03");
        assert_lexeme("04", b"04");
        assert_lexeme("05", b"05");
        assert_lexeme("06", b"06");
        assert_lexeme("07", b"07");
        assert_lexeme("0377", b"0377");
    }

    #[test]
    fn test_decimal() {
        assert_tok(TT::Int, b"0");
        assert_tok(TT::Int, b"1");
        assert_tok(TT::Int, b"2");
        assert_tok(TT::Int, b"3");
        assert_tok(TT::Int, b"4");
        assert_tok(TT::Int, b"5");
        assert_tok(TT::Int, b"6");
        assert_tok(TT::Int, b"7");
        assert_tok(TT::Int, b"8");
        assert_tok(TT::Int, b"9");
        assert_tok(TT::Int, b"127");

        assert_lexeme("0", b"0");
        assert_lexeme("1", b"1");
        assert_lexeme("2", b"2");
        assert_lexeme("3", b"3");
        assert_lexeme("4", b"4");
        assert_lexeme("5", b"5");
        assert_lexeme("6", b"6");
        assert_lexeme("7", b"7");
        assert_lexeme("8", b"8");
        assert_lexeme("9", b"9");
        assert_lexeme("127", b"127");
    }

    #[test]
    fn test_float_literal() {
        assert_tok(TT::Dot, b".");
        assert_tok(TT::Float, b"3.");
        assert_tok(TT::Float, b".3");
        assert_tok(TT::Float, b"2.3");
        assert_tok(TT::Float, b"0.3");

        assert_lexeme("3.", b"3.");
        assert_lexeme(".3", b".3");
        assert_lexeme("2.3", b"2.3");
        assert_lexeme("0.3", b"0.3");
    }

    #[test]
    fn test_last_tok() {
        let input = From::from(&(b"x += -4")[..]);
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

    #[test]
    fn test_semi_insertion() {
        assert_toks(&[TT::Id, TT::Semi], b"x\n");
        assert_toks(&[TT::Id, TT::Semi], b"x\n\n\n");
        assert_toks(&[TT::Id, TT::Semi], b"x // comment\n");

        assert_toks(&[TT::Blank, TT::Semi], b"_\n");
        assert_toks(&[TT::Blank, TT::Semi], b"_\n\n\n");
        assert_toks(&[TT::Blank, TT::Semi], b"_ // comment\n");

        assert_toks(&[TT::Int, TT::Semi], b"42\n");
        assert_toks(&[TT::Int, TT::Semi], b"42\n\n\n");
        assert_toks(&[TT::Int, TT::Semi], b"42 // comment\n");

        assert_toks(&[TT::IntHex, TT::Semi], b"0x1f\n");
        assert_toks(&[TT::IntHex, TT::Semi], b"0x1f\n\n\n");
        assert_toks(&[TT::IntHex, TT::Semi], b"0x1f // comment\n");

        assert_toks(&[TT::Float, TT::Semi], b"3.14\n");
        assert_toks(&[TT::Float, TT::Semi], b"3.14\n\n\n");
        assert_toks(&[TT::Float, TT::Semi], b"3.14 // comment\n");

        assert_toks(&[TT::Break, TT::Semi], b"break\n");
        assert_toks(&[TT::Break, TT::Semi], b"break\n\n\n");
        assert_toks(&[TT::Break, TT::Semi], b"break // comment\n");

        assert_toks(&[TT::Continue, TT::Semi], b"continue\n");
        assert_toks(&[TT::Continue, TT::Semi], b"continue\n\n\n");
        assert_toks(&[TT::Continue, TT::Semi], b"continue // comment\n");

        assert_toks(&[TT::Return, TT::Semi], b"return\n");
        assert_toks(&[TT::Return, TT::Semi], b"return\n\n\n");
        assert_toks(&[TT::Return, TT::Semi], b"return // comment\n");

        assert_toks(&[TT::Incr, TT::Semi], b"++\n");
        assert_toks(&[TT::Incr, TT::Semi], b"++\n\n\n");
        assert_toks(&[TT::Incr, TT::Semi], b"++ // comment\n");

        assert_toks(&[TT::Decr, TT::Semi], b"--\n");
        assert_toks(&[TT::Decr, TT::Semi], b"--\n\n\n");
        assert_toks(&[TT::Decr, TT::Semi], b"-- // comment\n");

        assert_toks(&[TT::RParen, TT::Semi], b")\n");
        assert_toks(&[TT::RParen, TT::Semi], b")\n\n\n");
        assert_toks(&[TT::RParen, TT::Semi], b") // comment\n");

        assert_toks(&[TT::RBracket, TT::Semi], b"]\n");
        assert_toks(&[TT::RBracket, TT::Semi], b"]\n\n\n");
        assert_toks(&[TT::RBracket, TT::Semi], b"] // comment\n");

        assert_toks(&[TT::RBrace, TT::Semi], b"}\n");
        assert_toks(&[TT::RBrace, TT::Semi], b"}\n\n\n");
        assert_toks(&[TT::RBrace, TT::Semi], b"} // comment\n");
    }

    #[test]
    fn test_no_semi_insertion() {
        assert_toks(&[TT::Case], b"case\n");
        assert_toks(&[TT::Default], b"default\n");
        assert_toks(&[TT::Else], b"else\n");
        assert_toks(&[TT::For], b"for\n");
        assert_toks(&[TT::Func], b"func\n");
        assert_toks(&[TT::If], b"if\n");
        assert_toks(&[TT::Package], b"package\n");
        assert_toks(&[TT::Struct], b"struct\n");
        assert_toks(&[TT::Switch], b"switch\n");
        assert_toks(&[TT::Type], b"type\n");
        assert_toks(&[TT::Var], b"var\n");
        assert_toks(&[TT::Append], b"append\n");
        assert_toks(&[TT::Print], b"print\n");
        assert_toks(&[TT::Println], b"println\n");


        assert_toks(&[TT::Plus], b"+\n");
        assert_toks(&[TT::Minus], b"-\n");
        assert_toks(&[TT::Star], b"*\n");
        assert_toks(&[TT::Slash], b"/\n");
        assert_toks(&[TT::Percent], b"%\n");
        assert_toks(&[TT::PlusEq], b"+=\n");
        assert_toks(&[TT::MinusEq], b"-=\n");
        assert_toks(&[TT::StarEq], b"*=\n");
        assert_toks(&[TT::SlashEq], b"/=\n");
        assert_toks(&[TT::PercentEq], b"%=\n");
        assert_toks(&[TT::Bitand], b"&\n");
        assert_toks(&[TT::Bitor], b"|\n");
        assert_toks(&[TT::Bitnot], b"^\n");
        assert_toks(&[TT::BitandEq], b"&=\n");
        assert_toks(&[TT::BitorEq], b"|=\n");
        assert_toks(&[TT::LeftShift], b"<<\n");
        assert_toks(&[TT::RightShift], b">>\n");
        assert_toks(&[TT::LeftShiftEq], b"<<=\n");
        assert_toks(&[TT::RightShiftEq], b">>=\n");
        assert_toks(&[TT::BitClear], b"&^\n");
        assert_toks(&[TT::And], b"&&\n");
        assert_toks(&[TT::Or], b"||\n");
        assert_toks(&[TT::Not], b"!\n");
        assert_toks(&[TT::Eq], b"==\n");
        assert_toks(&[TT::Ne], b"!=\n");
        assert_toks(&[TT::Lt], b"<\n");
        assert_toks(&[TT::Le], b"<=\n");
        assert_toks(&[TT::Gt], b">\n");
        assert_toks(&[TT::Ge], b">=\n");
        assert_toks(&[TT::LParen], b"(\n");
        assert_toks(&[TT::LBracket], b"[\n");
        assert_toks(&[TT::LBrace], b"{\n");
        assert_toks(&[TT::Comma], b",\n");
        assert_toks(&[TT::Dot], b".\n");
        assert_toks(&[TT::Semi], b";\n");
        assert_toks(&[TT::Colon], b":\n");
    }
}
