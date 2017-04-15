extern crate gore;
#[macro_use] extern crate matches;

use gore::token::TokenType as TT;
use gore::error::GoreErrorType as ET;
use gore::scanner::Scanner;

fn tok(src: &[u8]) -> Result<TT, ET> {
    let mut scanner = Scanner::new("-".to_string(), src.to_vec());
    let tok_res = scanner.next();
    tok_res
        .map(|tok| tok.ty)
        .map_err(|err| err.ty)
}

fn lexeme(src: &[u8]) -> Result<Option<String>, ET> {
    let mut scanner = Scanner::new("-".to_string(), src.to_vec());
    let tok_opt = scanner.next();
    tok_opt
        .map(|tok| tok.lexeme)
        .map_err(|err| err.ty)
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

#[test]
fn test_invalid_characters() {
    assert!(matches!(tok(b"#"), Err(ET::UnrecognizedCharacter(_))));
    assert!(matches!(tok(b"$"), Err(ET::UnrecognizedCharacter(_))));
    assert!(matches!(tok(b"?"), Err(ET::UnrecognizedCharacter(_))));
    assert!(matches!(tok(b"~"), Err(ET::UnrecognizedCharacter(_))));
}

#[test]
fn test_ops_and_punc() {
    assert!(matches!(tok(b":=")  , Ok(TT::ColonEq)));
    assert!(matches!(tok(b"+=")  , Ok(TT::PlusEq)));
    assert!(matches!(tok(b"++")  , Ok(TT::Incr)));
    assert!(matches!(tok(b"+")   , Ok(TT::Plus)));
    assert!(matches!(tok(b"-=")  , Ok(TT::MinusEq)));
    assert!(matches!(tok(b"--")  , Ok(TT::Decr)));
    assert!(matches!(tok(b"-")   , Ok(TT::Minus)));
    assert!(matches!(tok(b"*=")  , Ok(TT::StarEq)));
    assert!(matches!(tok(b"*")   , Ok(TT::Star)));
    assert!(matches!(tok(b"/=")  , Ok(TT::SlashEq)));
    assert!(matches!(tok(b"/")   , Ok(TT::Slash)));
    assert!(matches!(tok(b"%=")  , Ok(TT::PercentEq)));
    assert!(matches!(tok(b"%")   , Ok(TT::Percent)));
    assert!(matches!(tok(b"^")   , Ok(TT::Bitnot)));
    assert!(matches!(tok(b"&^")  , Ok(TT::BitClear)));
    assert!(matches!(tok(b"&=")  , Ok(TT::BitandEq)));
    assert!(matches!(tok(b"&")   , Ok(TT::Bitand)));
    assert!(matches!(tok(b"|=")  , Ok(TT::BitorEq)));
    assert!(matches!(tok(b"|")   , Ok(TT::Bitor)));
    assert!(matches!(tok(b"<<=") , Ok(TT::LeftShiftEq)));
    assert!(matches!(tok(b"<<")  , Ok(TT::LeftShift)));
    assert!(matches!(tok(b">>=") , Ok(TT::RightShiftEq)));
    assert!(matches!(tok(b">>")  , Ok(TT::RightShift)));
    assert!(matches!(tok(b"&&")  , Ok(TT::And)));
    assert!(matches!(tok(b"||")  , Ok(TT::Or)));
    assert!(matches!(tok(b"!")   , Ok(TT::Not)));
    assert!(matches!(tok(b"==")  , Ok(TT::Eq)));
    assert!(matches!(tok(b"!=")  , Ok(TT::Ne)));
    assert!(matches!(tok(b"<=")  , Ok(TT::Le)));
    assert!(matches!(tok(b"<")   , Ok(TT::Lt)));
    assert!(matches!(tok(b">=")  , Ok(TT::Ge)));
    assert!(matches!(tok(b">")   , Ok(TT::Gt)));
    assert!(matches!(tok(b"(")   , Ok(TT::LParen)));
    assert!(matches!(tok(b")")   , Ok(TT::RParen)));
    assert!(matches!(tok(b"[")   , Ok(TT::LBracket)));
    assert!(matches!(tok(b"]")   , Ok(TT::RBracket)));
    assert!(matches!(tok(b"{")   , Ok(TT::LBrace)));
    assert!(matches!(tok(b"}")   , Ok(TT::RBrace)));
    assert!(matches!(tok(b",")   , Ok(TT::Comma)));
    assert!(matches!(tok(b".")   , Ok(TT::Dot)));
    assert!(matches!(tok(b";")   , Ok(TT::Semi)));
    assert!(matches!(tok(b":")   , Ok(TT::Colon)));
}

#[test]
fn test_skip_whitespace_and_comments() {
    assert!(matches!(tok(b"."), Ok(TT::Dot)));
    assert!(matches!(tok(b" ."), Ok(TT::Dot)));
    assert!(matches!(tok(b"\t."), Ok(TT::Dot)));
    assert!(matches!(tok(b"\n."), Ok(TT::Dot)));
    assert!(matches!(tok(b"// comment\n."), Ok(TT::Dot)));
    assert!(matches!(tok(b"// comment 1\n  // comment 2\n\n."), Ok(TT::Dot)));
    assert!(matches!(tok(b"/* comment \n comment */   ."), Ok(TT::Dot)));
    assert!(matches!(tok(b"// only a line comment\n"), Ok(TT::Eof)));
    assert!(matches!(tok(b"// only a line comment"), Ok(TT::Eof)));
    assert!(matches!(tok(b"/* unfinished business"), Err(ET::TrailingBlockComment)));
}

#[test]
fn test_keywords() {
    assert!(matches!(tok(b"break"), Ok(TT::Break)));
    assert!(matches!(tok(b"case"), Ok(TT::Case)));
    assert!(matches!(tok(b"continue"), Ok(TT::Continue)));
    assert!(matches!(tok(b"default"), Ok(TT::Default)));
    assert!(matches!(tok(b"else"), Ok(TT::Else)));
    assert!(matches!(tok(b"for"), Ok(TT::For)));
    assert!(matches!(tok(b"func"), Ok(TT::Func)));
    assert!(matches!(tok(b"if"), Ok(TT::If)));
    assert!(matches!(tok(b"package"), Ok(TT::Package)));
    assert!(matches!(tok(b"return"), Ok(TT::Return)));
    assert!(matches!(tok(b"struct"), Ok(TT::Struct)));
    assert!(matches!(tok(b"switch"), Ok(TT::Switch)));
    assert!(matches!(tok(b"type"), Ok(TT::Type)));
    assert!(matches!(tok(b"var"), Ok(TT::Var)));
    assert!(matches!(tok(b"append"), Ok(TT::Append)));
    assert!(matches!(tok(b"print"), Ok(TT::Print)));
    assert!(matches!(tok(b"println"), Ok(TT::Println)));
}

#[test]
fn test_ids() {
    assert!(matches!(tok(b"foo"), Ok(TT::Id)));
    assert!(matches!(tok(b"Foo"), Ok(TT::Id)));
    assert!(matches!(tok(b"_foo"), Ok(TT::Id)));
    assert!(matches!(tok(b"_1"), Ok(TT::Id)));
    assert!(matches!(tok(b"__LINE__"), Ok(TT::Id)));
    assert!(matches!(tok(b"_"), Ok(TT::Blank)));

    assert!(matches!(lexeme(b"foo"), Ok(Some(ref s)) if s == "foo"));
    assert!(matches!(lexeme(b"_"), Ok(None)));
}

#[test]
fn test_hex() {
    assert!(matches!(tok(b"0x0"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x1"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x2"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x3"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x4"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x5"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x6"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x7"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x8"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0x9"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xa"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xb"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xc"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xd"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xe"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0xf"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XA"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XB"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XC"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XD"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XE"), Ok(TT::IntHex)));
    assert!(matches!(tok(b"0XF"), Ok(TT::IntHex)));

    assert!(matches!(lexeme(b"0x0"), Ok(Some(ref s)) if s == "0"));
    assert!(matches!(lexeme(b"0x1"), Ok(Some(ref s)) if s == "1"));
    assert!(matches!(lexeme(b"0x2"), Ok(Some(ref s)) if s == "2"));
    assert!(matches!(lexeme(b"0x3"), Ok(Some(ref s)) if s == "3"));
    assert!(matches!(lexeme(b"0x4"), Ok(Some(ref s)) if s == "4"));
    assert!(matches!(lexeme(b"0x5"), Ok(Some(ref s)) if s == "5"));
    assert!(matches!(lexeme(b"0x6"), Ok(Some(ref s)) if s == "6"));
    assert!(matches!(lexeme(b"0x7"), Ok(Some(ref s)) if s == "7"));
    assert!(matches!(lexeme(b"0x8"), Ok(Some(ref s)) if s == "8"));
    assert!(matches!(lexeme(b"0x9"), Ok(Some(ref s)) if s == "9"));
    assert!(matches!(lexeme(b"0xa"), Ok(Some(ref s)) if s == "a"));
    assert!(matches!(lexeme(b"0xb"), Ok(Some(ref s)) if s == "b"));
    assert!(matches!(lexeme(b"0xc"), Ok(Some(ref s)) if s == "c"));
    assert!(matches!(lexeme(b"0xd"), Ok(Some(ref s)) if s == "d"));
    assert!(matches!(lexeme(b"0xe"), Ok(Some(ref s)) if s == "e"));
    assert!(matches!(lexeme(b"0xf"), Ok(Some(ref s)) if s == "f"));
    assert!(matches!(lexeme(b"0XA"), Ok(Some(ref s)) if s == "A"));
    assert!(matches!(lexeme(b"0XB"), Ok(Some(ref s)) if s == "B"));
    assert!(matches!(lexeme(b"0XC"), Ok(Some(ref s)) if s == "C"));
    assert!(matches!(lexeme(b"0XD"), Ok(Some(ref s)) if s == "D"));
    assert!(matches!(lexeme(b"0XE"), Ok(Some(ref s)) if s == "E"));
    assert!(matches!(lexeme(b"0XF"), Ok(Some(ref s)) if s == "F"));

    assert!(matches!(tok(b"0x"), Err(ET::EmptyHexLiteral)));
    assert!(matches!(tok(b"0X"), Err(ET::EmptyHexLiteral)));
}

#[test]
fn test_octal() {
    assert!(matches!(tok(b"0"),  Ok(TT::Int)));
    assert!(matches!(tok(b"01"), Ok(TT::Int)));
    assert!(matches!(tok(b"02"), Ok(TT::Int)));
    assert!(matches!(tok(b"03"), Ok(TT::Int)));
    assert!(matches!(tok(b"04"), Ok(TT::Int)));
    assert!(matches!(tok(b"05"), Ok(TT::Int)));
    assert!(matches!(tok(b"06"), Ok(TT::Int)));
    assert!(matches!(tok(b"07"), Ok(TT::Int)));
    assert!(matches!(tok(b"0377"), Ok(TT::Int)));

    assert!(matches!(lexeme(b"0"), Ok(Some(ref s)) if s == "0"));
    assert!(matches!(lexeme(b"01"), Ok(Some(ref s)) if s == "01"));
    assert!(matches!(lexeme(b"02"), Ok(Some(ref s)) if s == "02"));
    assert!(matches!(lexeme(b"03"), Ok(Some(ref s)) if s == "03"));
    assert!(matches!(lexeme(b"04"), Ok(Some(ref s)) if s == "04"));
    assert!(matches!(lexeme(b"05"), Ok(Some(ref s)) if s == "05"));
    assert!(matches!(lexeme(b"06"), Ok(Some(ref s)) if s == "06"));
    assert!(matches!(lexeme(b"07"), Ok(Some(ref s)) if s == "07"));
    assert!(matches!(lexeme(b"0377"), Ok(Some(ref s)) if s == "0377"));
}

#[test]
fn test_decimal() {
    assert!(matches!(tok(b"0"), Ok(TT::Int)));
    assert!(matches!(tok(b"1"), Ok(TT::Int)));
    assert!(matches!(tok(b"2"), Ok(TT::Int)));
    assert!(matches!(tok(b"3"), Ok(TT::Int)));
    assert!(matches!(tok(b"4"), Ok(TT::Int)));
    assert!(matches!(tok(b"5"), Ok(TT::Int)));
    assert!(matches!(tok(b"6"), Ok(TT::Int)));
    assert!(matches!(tok(b"7"), Ok(TT::Int)));
    assert!(matches!(tok(b"8"), Ok(TT::Int)));
    assert!(matches!(tok(b"9"), Ok(TT::Int)));
    assert!(matches!(tok(b"127"), Ok(TT::Int)));

    assert!(matches!(lexeme(b"0"), Ok(Some(ref s)) if s == "0"));
    assert!(matches!(lexeme(b"1"), Ok(Some(ref s)) if s == "1"));
    assert!(matches!(lexeme(b"2"), Ok(Some(ref s)) if s == "2"));
    assert!(matches!(lexeme(b"3"), Ok(Some(ref s)) if s == "3"));
    assert!(matches!(lexeme(b"4"), Ok(Some(ref s)) if s == "4"));
    assert!(matches!(lexeme(b"5"), Ok(Some(ref s)) if s == "5"));
    assert!(matches!(lexeme(b"6"), Ok(Some(ref s)) if s == "6"));
    assert!(matches!(lexeme(b"7"), Ok(Some(ref s)) if s == "7"));
    assert!(matches!(lexeme(b"8"), Ok(Some(ref s)) if s == "8"));
    assert!(matches!(lexeme(b"9"), Ok(Some(ref s)) if s == "9"));
    assert!(matches!(lexeme(b"127"), Ok(Some(ref s)) if s == "127"));
}

#[test]
fn test_float_literal() {
    assert!(matches!(tok(b"."), Ok(TT::Dot)));
    assert!(matches!(tok(b"3."), Ok(TT::Float)));
    assert!(matches!(tok(b".3"), Ok(TT::Float)));
    assert!(matches!(tok(b"2.3"), Ok(TT::Float)));
    assert!(matches!(tok(b"0.3"), Ok(TT::Float)));

    assert!(matches!(lexeme(b"3."), Ok(Some(ref s)) if s == "3."));
    assert!(matches!(lexeme(b".3"), Ok(Some(ref s)) if s == ".3"));
    assert!(matches!(lexeme(b"2.3"), Ok(Some(ref s)) if s == "2.3"));
    assert!(matches!(lexeme(b"0.3"), Ok(Some(ref s)) if s == "0.3"));
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
    assert!(matches!(tok(b"\"\""), Ok(TT::String)));
    assert!(matches!(tok(b"\"hello\""), Ok(TT::String)));
    assert!(matches!(tok(b"\" \\a \\b \\f \\n \\r \\t \\v \\\\ \\\" \""), Ok(TT::String)));

    assert!(matches!(tok(b"\"hello"), Err(ET::TrailingString)));
    assert!(matches!(tok(b"\"\\p\""), Err(ET::InvalidEscape(_))));
    assert!(matches!(tok(b" \" \n \" "), Err(ET::NewlineInString)));
}

#[test]
fn test_raw_string() {
    assert!(matches!(tok(b"``"), Ok(TT::String)));
    assert!(matches!(tok(b"`hello`"), Ok(TT::String)));;
    assert!(matches!(tok(b"`hello\nworld`"), Ok(TT::String)));;

    assert!(matches!(lexeme(b"`\\r`"), Ok(Some(ref s)) if s == ""));

    assert!(matches!(tok(b"`hello"), Err(ET::TrailingString)));
}

#[test]
fn test_rune() {
    assert!(matches!(tok(b"'a'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\a'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\b'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\f'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\n'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\r'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\t'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\v'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\\\'"), Ok(TT::Rune)));
    assert!(matches!(tok(b"'\\''"), Ok(TT::Rune)));

    assert!(matches!(tok(b"''"), Err(ET::EmptyRune)));
    assert!(matches!(tok(b"'\n'"), Err(ET::NewlineInRune)));
    assert!(matches!(tok(b"'"), Err(ET::TrailingRune)));
    assert!(matches!(tok(b"'x"), Err(ET::TrailingRune)));
    assert!(matches!(tok(b"'\\p'"), Err(ET::InvalidEscape(_))));
    assert!(matches!(tok(b"'xx'"), Err(ET::TrailingRune)));
}
