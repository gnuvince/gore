extern crate gore;

use gore::token::Token;
use gore::token::TokenType as TT;
use gore::error::ErrorType as ET;
use gore::scanner::Scanner;

fn assert_tok(expected_ty: TT, src: &[u8]) {
    let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
    let mut scanner = Scanner::new("-".to_string(), src_vec);
    let tok_opt = scanner.next();
    let tok_ty = tok_opt.map(|tok| tok.ty).unwrap_or(TT::Eof);
    assert_eq!(tok_ty, expected_ty);
}

fn assert_toks(expected_tys: &[TT], src: &[u8]) {
    let src_vec: Vec<u8> = src.iter().map(|b| *b).collect();
    let mut scanner = Scanner::new("-".to_string(), src_vec);
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
    let mut scanner = Scanner::new("-".to_string(), src_vec);
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
    let mut scanner = Scanner::new("-".to_string(), src_vec);
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
    assert_tok(TT::ColonEq, b":=");
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
    assert_tok(TT::IntOct, b"0");
    assert_tok(TT::IntOct, b"01");
    assert_tok(TT::IntOct, b"02");
    assert_tok(TT::IntOct, b"03");
    assert_tok(TT::IntOct, b"04");
    assert_tok(TT::IntOct, b"05");
    assert_tok(TT::IntOct, b"06");
    assert_tok(TT::IntOct, b"07");
    assert_tok(TT::IntOct, b"0377");

    assert_lexeme("0", b"0");
    assert_lexeme("01", b"01");
    assert_lexeme("02", b"02");
    assert_lexeme("03", b"03");
    assert_lexeme("04", b"04");
    assert_lexeme("05", b"05");
    assert_lexeme("06", b"06");
    assert_lexeme("07", b"07");
    assert_lexeme("0377", b"0377");

    assert_err(ET::MalformedOctLiteral, b"08");
    assert_err(ET::MalformedOctLiteral, b"09");
}

#[test]
fn test_decimal() {
    assert_tok(TT::IntOct, b"0");
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
fn test_semi_insertion() {

    assert_toks(&[TT::Id, TT::Semi], b"x");
    assert_toks(&[TT::Id, TT::Semi], b"x\n");
    assert_toks(&[TT::Id, TT::Semi], b"x\n\n\n");
    assert_toks(&[TT::Id, TT::Semi], b"x // comment\n");

    assert_toks(&[TT::Blank, TT::Semi], b"_");
    assert_toks(&[TT::Blank, TT::Semi], b"_\n");
    assert_toks(&[TT::Blank, TT::Semi], b"_\n\n\n");
    assert_toks(&[TT::Blank, TT::Semi], b"_ // comment\n");

    assert_toks(&[TT::Int, TT::Semi], b"42");
    assert_toks(&[TT::Int, TT::Semi], b"42\n");
    assert_toks(&[TT::Int, TT::Semi], b"42\n\n\n");
    assert_toks(&[TT::Int, TT::Semi], b"42 // comment\n");

    assert_toks(&[TT::IntHex, TT::Semi], b"0x1f");
    assert_toks(&[TT::IntHex, TT::Semi], b"0x1f\n");
    assert_toks(&[TT::IntHex, TT::Semi], b"0x1f\n\n\n");
    assert_toks(&[TT::IntHex, TT::Semi], b"0x1f // comment\n");

    assert_toks(&[TT::Float, TT::Semi], b"3.14");
    assert_toks(&[TT::Float, TT::Semi], b"3.14\n");
    assert_toks(&[TT::Float, TT::Semi], b"3.14\n\n\n");
    assert_toks(&[TT::Float, TT::Semi], b"3.14 // comment\n");

    assert_toks(&[TT::Break, TT::Semi], b"break");
    assert_toks(&[TT::Break, TT::Semi], b"break\n");
    assert_toks(&[TT::Break, TT::Semi], b"break\n\n\n");
    assert_toks(&[TT::Break, TT::Semi], b"break // comment\n");

    assert_toks(&[TT::Continue, TT::Semi], b"continue");
    assert_toks(&[TT::Continue, TT::Semi], b"continue\n");
    assert_toks(&[TT::Continue, TT::Semi], b"continue\n\n\n");
    assert_toks(&[TT::Continue, TT::Semi], b"continue // comment\n");

    assert_toks(&[TT::Return, TT::Semi], b"return");
    assert_toks(&[TT::Return, TT::Semi], b"return\n");
    assert_toks(&[TT::Return, TT::Semi], b"return\n\n\n");
    assert_toks(&[TT::Return, TT::Semi], b"return // comment\n");

    assert_toks(&[TT::Incr, TT::Semi], b"++");
    assert_toks(&[TT::Incr, TT::Semi], b"++\n");
    assert_toks(&[TT::Incr, TT::Semi], b"++\n\n\n");
    assert_toks(&[TT::Incr, TT::Semi], b"++ // comment\n");

    assert_toks(&[TT::Decr, TT::Semi], b"--");
    assert_toks(&[TT::Decr, TT::Semi], b"--\n");
    assert_toks(&[TT::Decr, TT::Semi], b"--\n\n\n");
    assert_toks(&[TT::Decr, TT::Semi], b"-- // comment\n");

    assert_toks(&[TT::RParen, TT::Semi], b")");
    assert_toks(&[TT::RParen, TT::Semi], b")\n");
    assert_toks(&[TT::RParen, TT::Semi], b")\n\n\n");
    assert_toks(&[TT::RParen, TT::Semi], b") // comment\n");

    assert_toks(&[TT::RBracket, TT::Semi], b"]");
    assert_toks(&[TT::RBracket, TT::Semi], b"]\n");
    assert_toks(&[TT::RBracket, TT::Semi], b"]\n\n\n");
    assert_toks(&[TT::RBracket, TT::Semi], b"] // comment\n");

    assert_toks(&[TT::RBrace, TT::Semi], b"}");
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

    assert_toks(&[TT::Case], b"case");
    assert_toks(&[TT::Default], b"default");
    assert_toks(&[TT::Else], b"else");
    assert_toks(&[TT::For], b"for");
    assert_toks(&[TT::Func], b"func");
    assert_toks(&[TT::If], b"if");
    assert_toks(&[TT::Package], b"package");
    assert_toks(&[TT::Struct], b"struct");
    assert_toks(&[TT::Switch], b"switch");
    assert_toks(&[TT::Type], b"type");
    assert_toks(&[TT::Var], b"var");
    assert_toks(&[TT::Append], b"append");
    assert_toks(&[TT::Print], b"print");
    assert_toks(&[TT::Println], b"println");


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
    assert_toks(&[TT::Assign], b"=\n");
    assert_toks(&[TT::ColonEq], b":=\n");
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

    assert_toks(&[TT::Plus], b"+");
    assert_toks(&[TT::Minus], b"-");
    assert_toks(&[TT::Star], b"*");
    assert_toks(&[TT::Slash], b"/");
    assert_toks(&[TT::Percent], b"%");
    assert_toks(&[TT::PlusEq], b"+=");
    assert_toks(&[TT::MinusEq], b"-=");
    assert_toks(&[TT::StarEq], b"*=");
    assert_toks(&[TT::SlashEq], b"/=");
    assert_toks(&[TT::PercentEq], b"%=");
    assert_toks(&[TT::Assign], b"=");
    assert_toks(&[TT::ColonEq], b":=");
    assert_toks(&[TT::Bitand], b"&");
    assert_toks(&[TT::Bitor], b"|");
    assert_toks(&[TT::Bitnot], b"^");
    assert_toks(&[TT::BitandEq], b"&=");
    assert_toks(&[TT::BitorEq], b"|=");
    assert_toks(&[TT::LeftShift], b"<<");
    assert_toks(&[TT::RightShift], b">>");
    assert_toks(&[TT::LeftShiftEq], b"<<=");
    assert_toks(&[TT::RightShiftEq], b">>=");
    assert_toks(&[TT::BitClear], b"&^");
    assert_toks(&[TT::And], b"&&");
    assert_toks(&[TT::Or], b"||");
    assert_toks(&[TT::Not], b"!");
    assert_toks(&[TT::Eq], b"==");
    assert_toks(&[TT::Ne], b"!=");
    assert_toks(&[TT::Lt], b"<");
    assert_toks(&[TT::Le], b"<=");
    assert_toks(&[TT::Gt], b">");
    assert_toks(&[TT::Ge], b">=");
    assert_toks(&[TT::LParen], b"(");
    assert_toks(&[TT::LBracket], b"[");
    assert_toks(&[TT::LBrace], b"{");
    assert_toks(&[TT::Comma], b",");
    assert_toks(&[TT::Dot], b".");
    assert_toks(&[TT::Semi], b";");
    assert_toks(&[TT::Colon], b":");
}

#[test]
fn test_interpreted_string() {
    assert_tok(TT::String, b"\"\"");
    assert_tok(TT::String, b"\"hello\"");
    assert_tok(TT::String, b"\" \\a \\b \\f \\n \\r \\t \\v \\\\ \\\" \"");

    assert_err(ET::TrailingString, b"\"hello");
    assert_err(ET::InvalidEscape, b"\"\\p\"");
    assert_err(ET::NewlineInString, b" \" \n \" ");
}

#[test]
fn test_raw_string() {
    assert_tok(TT::String, b"``");
    assert_tok(TT::String, b"`hello`");
    assert_tok(TT::String, b"`hello\nworld`");

    assert_lexeme("", b"`\\r`");

    assert_err(ET::TrailingString, b"`hello");
}

#[test]
fn test_rune() {
    assert_tok(TT::Rune, b"'a'");
    assert_tok(TT::Rune, b"'\\a'");
    assert_tok(TT::Rune, b"'\\b'");
    assert_tok(TT::Rune, b"'\\f'");
    assert_tok(TT::Rune, b"'\\n'");
    assert_tok(TT::Rune, b"'\\r'");
    assert_tok(TT::Rune, b"'\\t'");
    assert_tok(TT::Rune, b"'\\v'");
    assert_tok(TT::Rune, b"'\\\\'");
    assert_tok(TT::Rune, b"'\\''");

    assert_err(ET::EmptyRune, b"''");
    assert_err(ET::NewlineInRune, b"'\n'");
    assert_err(ET::TrailingRune, b"'");
    assert_err(ET::TrailingRune, b"'x");
    assert_err(ET::InvalidEscape, b"'\\p'");
    assert_err(ET::TrailingRune, b"'xx'");
}
