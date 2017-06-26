use std::fmt;

use loc::Loc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Special tokens
    None, Eof,

    // Go Keywords
    Break, Case, Continue, Default, Else, For,
    Func, If, Package, Return, Struct, Switch,
    Type, Var,

    // Extra keywords for GoLite
    Append, Print, Println,

    // Literals
    Blank, Int, IntOct, IntHex, Float, String, Rune, Id,

    // Operators and punctuation
    Plus, Minus, Star, Slash, Percent,           // + - * / %
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq, // += -= *= /= %=
    Assign, ColonEq,                             // = :=
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
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;
        let s = match *self {
            Eof => "<eof>",
            None => "<none>",

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
            IntOct => "<int oct>",
            IntHex => "<int hex>",
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
            Assign => "=",
            ColonEq => ":=",
            Bitand => "&",
            Bitor => "|",
            Bitnot => "^",
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
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub loc: Loc,
    pub lexeme: Option<String>
}


impl Token {
    pub fn new(ty: TokenType, loc: Loc, lexeme: Option<String>) -> Token {
        Token {
            ty: ty, loc: loc, lexeme: lexeme
        }
    }

    pub fn is_eof(&self) -> bool {
        self.ty == TokenType::Eof
    }
}
