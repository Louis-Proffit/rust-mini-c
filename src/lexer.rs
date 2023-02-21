use logos::Logos;

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
    #[regex("[1-9][0-9]+", | lex | lex.slice().parse())]
    DecimalConstant(i32),
    #[regex("0[xX][0-9a-fA-F]+", | lex | lex.slice().parse())]
    HexConstant(i32),
    #[regex("0[1-7][0-7]*", | lex | lex.slice().parse())]
    OctalConstant(i32),

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,
    #[regex(r"//[^\n\r]+(?:[\n\r]|\*\))", logos::skip)]
    LineComment,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::lexer::Token;

    #[test]
    fn test_all() {
        let to_test = "if x = 3 struct sizeof // zeogueztojb \n zefou _3 /* eroubg */ while";

        let mut lexer = Token::lexer(to_test);

        while let Some(t) = lexer.next() {
            println!("{:?}: {}", t, lexer.slice())
        }
    }
}