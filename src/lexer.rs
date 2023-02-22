use std::str::FromStr;
use logos::{Lexer, Logos};

fn parse_hexadecimal(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();
    let without_prefix = match slice.strip_prefix("0x") {
        None => match slice.strip_prefix("0X") {
            None => None,
            Some(striped) => Some(striped)
        },
        Some(striped) => Some(striped)
    }?;
    let z = i32::from_str_radix(without_prefix, 16).ok()?;
    Some(z)
}

fn parse_char(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();
    let slice = slice.strip_prefix("'")?;
    let slice = slice.strip_suffix("'")?;
    let c = char::from_str(slice).ok()?;
    Some((c as u32) as i32)
}

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token {
    #[token("struct")]
    Struct,
    #[token("int")]
    Int,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("sizeof")]
    Sizeof,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Div,
    #[token("!")]
    Bang,
    #[token("->")]
    Arrow,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    BangEq,
    #[token("<=")]
    Le,
    #[token("<")]
    Lt,
    #[token(">=")]
    Ge,
    #[token(">")]
    Gt,
    #[token("&&")]
    AmpersandAmpersand,
    #[token("||")]
    VerticalBarVerticalBar,
    #[token("{")]
    Lbrace,
    #[token("}")]
    Rbrace,
    #[token("(")]
    Lpar,
    #[token(")")]
    Rpar,
    #[token(";")]
    Semicolon,
    #[token("*")]
    Star,
    #[token(",")]
    Comma,
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident,
    #[regex("[1-9][0-9]*", | lex | lex.slice().parse())]
    DecimalConstant(i32),
    #[regex("0[xX][0-9a-fA-F]+", parse_hexadecimal)]
    HexConstant(i32),
    #[regex("0[0-7]*", | lex | lex.slice().parse())]
    OctalConstant(i32),
    #[regex("'.'", parse_char)]
    CharConstant(i32),
    #[regex(r"'\\n'")]
    NewlineConstant,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,
    #[regex(r"//[^\n\r]*(?:[\n\r]|\*\))", logos::skip)]
    LineComment,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    Error,
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::lexer::Token;

    fn _test_hexadecimal_valid(to_test: &str, result: i32) {
        _test_value(to_test, vec![Token::HexConstant(result)])
    }

    fn _test_value(to_test: &str, result: Vec<Token>) {
        let lexer = Token::lexer(to_test);

        for (expected, actual) in result.iter().zip(lexer) {
            assert_eq!(expected, &actual)
        }
    }

    macro_rules! test_hexadecimal_valid {
        ($name:ident: $value:literal -> $result:literal) => {
            #[test]
            fn $name() {
                _test_hexadecimal_valid($value, $result)
            }
        };
    }

    test_hexadecimal_valid!(hex_lower_0: "0x0" -> 0);
    test_hexadecimal_valid!(hex_lower_1: "0x1" -> 1);
    test_hexadecimal_valid!(hex_uppper_0: "0X0" -> 0);
    test_hexadecimal_valid!(hex_upper_1: "0X1" -> 1);

    #[test]
    fn show() {
        let to_test = "int m() { return 1==2 && 3==4 || !(5>=6); }\n\n";
        let mut tokens = Token::lexer(to_test);

        while let Some(x) = tokens.next() {
            println!("{:?}", x)
        }
    }

    #[test]
    fn test_all() {
        let string = "struct int if else \
        while return sizeof + - / * !  -> && \
        || != == <= >= < > ( ) { } ; , x\
         /* zzzz */ // zzzz \n 0 0x0 56 'a' '\\n'";
        _test_value(string, vec![
            Token::Struct,
            Token::Int,
            Token::If,
            Token::Else,
            Token::While,
            Token::Return,
            Token::Sizeof,
            Token::Plus,
            Token::Minus,
            Token::Div,
            Token::Star,
            Token::Bang,
            Token::Arrow,
            Token::AmpersandAmpersand,
            Token::VerticalBarVerticalBar,
            Token::BangEq,
            Token::EqEq,
            Token::Le,
            Token::Ge,
            Token::Lt,
            Token::Gt,
            Token::Lpar,
            Token::Rpar,
            Token::Lbrace,
            Token::Rbrace,
            Token::Semicolon,
            Token::Comma,
            Token::Ident,
            Token::OctalConstant(0),
            Token::HexConstant(0),
            Token::DecimalConstant(56),
            Token::CharConstant(97),
            Token::NewlineConstant
        ])
    }
}