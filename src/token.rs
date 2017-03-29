use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Go Keywords
    Break, Case, Continue, Default, Else, For,
    Func, If, Package, Return, Struct, Switch,
    Type, Var,

    // Extra keywords for GoLite
    Append, Print, Println,

    // Literals
    Blank, Int, Float, String, Rune, Id,

    // Operators and punctuation
    Plus, Minus, Star, Slash, Percent,           // + - * / %
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq, // += -= *= /= %=
    Bitand, Bitor, Bitnot,                       // & | ^
    BitandEq, BitorEq,                           // &= |=
    LeftShift, RightShift, BitClear,             // << >> &^
    LeftShiftEq, RightShiftEq,                   // <<= >>=
    And, Or, Not, Incr, Decr,                    // && || ! ++ --
    Eq, Ne, Lt, Le, Gt, Ge,                      // == != < <= > >=
    LParen, RParen,                              // ( )
    LBracket, RBracket,                          // [ ]
    LBrace, RBrace,                              // { }
    Comma, Dot, Semi, Colon,                     // , . ; :
    Eof
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;
        let s = match *self {
            Break => "break",
            Case => "case",
            Continue => "continue",
            Default => "default",
            Else => "else",
            For => "for",
            Func => "func",
            If => "if",
            Package => "package",
            Return => "return",
            Struct => "struct",
            Switch => "switch",
            Type => "type",
            Var => "var",
            Append => "append",
            Print => "print",
            Println => "println",

            Blank => "_",
            Int => "<int>",
            Float => "<float>",
            String => "<string>",
            Rune => "<rune>",
            Id => "<id>",

            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            Bitand => "&",
            Bitor => "|",
            Bitnot => "!",
            BitandEq => "&=",
            BitorEq => "|=",
            LeftShift => "<<",
            RightShift => ">>",
            LeftShiftEq => "<<=",
            RightShiftEq => ">>=",
            BitClear => "&^",
            And => "&&",
            Or => "||",
            Not => "!",
            Incr => "++",
            Decr => "--",
            Eq => "==",
            Ne => "!=",
            Lt => "<",
            Le => "<=",
            Gt => ">",
            Ge => ">=",
            LParen => "(",
            RParen => ")",
            LBracket => "[",
            RBracket => "]",
            LBrace => "{",
            RBrace => "}",
            Comma => ",",
            Dot => ".",
            Semi => ";",
            Colon => ":",
            Eof => "<eof>"
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug)]
struct Token {
    ty: TokenType,
    line: usize,
    col: usize,
    lexeme: Option<String>
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<< {} ({}, {}) {} >>",
               self.ty, self.line, self.col,
               match self.lexeme {
                   None => "",
                   Some(ref s) => s
               }
        )
    }
}
