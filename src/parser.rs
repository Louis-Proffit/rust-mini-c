use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::{many0, separated_list0, separated_list1};
use logos_nom_bridge::{data_variant_parser, token_parser, Tokens};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::tuple;
use Token::Semicolon;
use crate::lexer::Token;
use crate::parser::structure::*;

pub type Input<'src> = Tokens<'src, Token>;

token_parser!(token: Token);

pub fn parse_file(input: Input) -> IResult<Input, File> {
    enum Decl<'a> {
        DeclStruct(Struct<'a>),
        DeclFun(Fun<'a>),
    }

    map(
        tuple((
            many0(
                alt((
                    map(decl_struct, |s| Decl::DeclStruct(s)),
                    map(decl_fun, |f| Decl::DeclFun(f)),
                ))),
            eof
        )),
        |(decls, _)| {
            let mut structs = vec![];
            let mut funs = vec![];
            for decl in decls {
                match decl {
                    Decl::DeclStruct(x) => structs.push(x),
                    Decl::DeclFun(x) => funs.push(x)
                }
            }
            File::new(funs, structs)
        },
    )(input)
}

fn eof(input: Input) -> IResult<Input, ()> {
    match input.peek() {
        None => Ok((input, ())),
        _ => Err(nom::Err::Error(ParseError::from_error_kind(input, ErrorKind::Eof)))
    }
}

fn decl_fun(input: Input) -> IResult<Input, Fun> {
    map(
        tuple((formal, Token::Lpar, separated_list0(Token::Comma, formal), Token::Rpar, block)),
        |(profile, _, args, _, body)| Fun::new(profile, args, body),
    )(input)
}

fn decl_struct(input: Input) -> IResult<Input, Struct> {
    map(
        tuple((Token::Struct, ident, Token::Lbrace, many0(decl_var), Token::Rbrace, Semicolon)),
        |(_, name, _, fields, _, _)| Struct::new(name, fields.into_iter().flatten().collect()),
    )(input)
}

fn star_ident(input: Input) -> IResult<Input, Ident> {
    map(tuple((Token::Star, ident)), |(_, x)| x)(input)
}

fn decl_var(input: Input) -> IResult<Input, Vec<Formal>> {
    alt((
        map(tuple((Token::Int, separated_list1(Token::Comma, ident), Semicolon)),
            |(_, idents, _)| idents.into_iter().map(|ident| Formal::new(ident, Typ::TInt)).collect(),
        ),
        map(tuple((Token::Struct, ident, separated_list1(Token::Comma, star_ident), Semicolon)),
            |(_, struct_ident, idents, _)| idents.into_iter().map(|ident| Formal::new(ident, Typ::TStruct(struct_ident))).collect(),
        ),
    ))(input)
}

fn formal(input: Input) -> IResult<Input, Formal> {
    alt((
        map(tuple((Token::Int, ident)), |(_, ident)| Formal::new(ident, Typ::TInt)),
        map(tuple((Token::Struct, ident, Token::Star, ident)), |(_, struct_ident, _, ident)| Formal::new(ident, Typ::TStruct(struct_ident)))
    ))(input)
}

fn ident(input: Input) -> IResult<Input, Ident> {
    map(Token::Ident, |x| x)(input)
}

fn block(input: Input) -> IResult<Input, Block> {
    enum BlockElement<'a> {
        Stmt(Stmt<'a>),
        DeclVar(Vec<Formal<'a>>),
    }

    map(
        tuple((
            Token::Lbrace,
            many0(
                alt((
                    map(decl_var, |x| BlockElement::DeclVar(x)),
                    map(stmt, |x| BlockElement::Stmt(x))
                ))
            ),
            Token::Rbrace
        )),
        |(_, elements, _)| {
            let mut vars = vec![];
            let mut stmts = vec![];
            for element in elements {
                match element {
                    BlockElement::Stmt(x) => stmts.push(x),
                    BlockElement::DeclVar(mut x) => vars.append(&mut x)
                }
            }
            Block::new(vars, stmts)
        },
    )(input)
}

fn stmt(input: Input) -> IResult<Input, Stmt> {
    let expr_stmt = tuple((opt(expr), Semicolon));
    let selection_stmt = tuple((Token::If, Token::Lpar, expr, Token::Rpar, stmt, opt(tuple((Token::Else, stmt)))));
    let iteration_stmt = tuple((Token::While, Token::Lpar, expr, Token::Rpar, stmt));
    let jump_stmt = tuple((Token::Return, opt(expr), Semicolon));

    alt((
        map(
            block,
            |x| Stmt::SBlock(x),
        ),
        map(
            expr_stmt,
            |(expr, _)| expr.map_or(Stmt::SSkip, |expr| Stmt::SExpr(expr)),
        ),
        map(
            selection_stmt,
            |(_, _, expr, _, e_if, e_else)| Stmt::SIf(expr, Box::new(e_if), e_else.map_or(Box::new(Stmt::SSkip), |(_, stmt)| Box::new(stmt))),
        ),
        map(
            iteration_stmt,
            |(_, _, expr, _, stmt)| Stmt::SWhile(expr, Box::new(stmt)),
        ),
        map(
            jump_stmt,
            |(_, expr, _)| expr.map_or(Stmt::SSkip, |expr| Stmt::SReturn(expr)),
        ),
    ))(input)
}

fn expr(input: Input) -> IResult<Input, Expr> {
    assign_expr(input)
}

fn primary_expr(input: Input) -> IResult<Input, Expr> {
    alt((
        map(integer, |x| Expr::EConst(x)),
        map(ident, |x| Expr::EVar(x)),
        map(tuple((Token::Lpar, expr, Token::Rpar)), |(_, x, _)| x),
    ))(input)
}

fn sizeof_expr(input: Input) -> IResult<Input, Expr> {
    map(
        tuple((Token::Sizeof, Token::Lpar, Token::Struct, ident, Token::Rpar)),
        |(_, _, _, ident, _)| Expr::ESizeof(ident),
    )(input)
}

fn postfix_expression(input: Input) -> IResult<Input, Expr> {
    enum PostfixSuffix<'a> {
        Call(Vec<Expr<'a>>),
        Access(Ident<'a>),
    }

    fn postfix_suffix(input: Input) -> IResult<Input, PostfixSuffix> {
        alt((
            map(
                tuple((Token::Lpar, separated_list0(Token::Comma, assign_expr), Token::Rpar)),
                |(_, exprs, _)| PostfixSuffix::Call(exprs),
            ),
            map(
                tuple((Token::Arrow, ident)),
                |(_, ident)| PostfixSuffix::Access(ident),
            )
        ))(input)
    }

    map(
        tuple((primary_expr, many0(postfix_suffix))),
        |(mut expr, suffixes)| {
            for suffix in suffixes {
                match suffix {
                    PostfixSuffix::Call(args) => expr = Expr::ECall(Box::new(expr), args),
                    PostfixSuffix::Access(ident) => expr = Expr::EArrow(Box::new(expr), ident)
                }
            }
            expr
        },
    )(input)
}

fn unary_op(input: Input) -> IResult<Input, Unop> {
    alt((
        map(Token::Minus, |_| Unop::UMinus),
        map(Token::Bang, |_| Unop::UNot),
    ))(input)
}

fn unary_expr(input: Input) -> IResult<Input, Expr> {
    alt((
        postfix_expression,
        map(tuple((unary_op, unary_expr)), |(op, expr)| Expr::EUnop(op, Box::new(expr))),
        sizeof_expr
    ))(input)
}

fn multiplicative_op(input: Input) -> IResult<Input, Binop> {
    alt((
        map(Token::Star, |_| Binop::BMul),
        map(Token::Div, |_| Binop::BDiv),
    ))(input)
}

fn additive_op(input: Input) -> IResult<Input, Binop> {
    alt((
        map(Token::Plus, |_| Binop::BAdd),
        map(Token::Minus, |_| Binop::BSub),
    ))(input)
}

fn relation_op(input: Input) -> IResult<Input, Binop> {
    alt((
        map(Token::Le, |_| Binop::BLe),
        map(Token::Lt, |_| Binop::BLt),
        map(Token::Ge, |_| Binop::BGe),
        map(Token::Gt, |_| Binop::BGt),
    ))(input)
}

fn eq_op(input: Input) -> IResult<Input, Binop> {
    alt((
        map(Token::EqEq, |_| Binop::BEq),
        map(Token::BangEq, |_| Binop::BNeq),
    ))(input)
}

fn multiplicative_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((unary_expr, many0(tuple((multiplicative_op, unary_expr)))))(input)?;

    for (binop, other_expr) in other_exprs {
        expr = Expr::EBinop(binop, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn additive_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((multiplicative_expr, many0(tuple((additive_op, multiplicative_expr)))))(input)?;

    for (binop, other_expr) in other_exprs {
        expr = Expr::EBinop(binop, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn relation_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((additive_expr, many0(tuple((relation_op, additive_expr)))))(input)?;

    for (binop, other_expr) in other_exprs {
        expr = Expr::EBinop(binop, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn eq_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((relation_expr, many0(tuple((eq_op, relation_expr)))))(input)?;

    for (binop, other_expr) in other_exprs {
        expr = Expr::EBinop(binop, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn and_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((eq_expr, many0(tuple((Token::AmpersandAmpersand, eq_expr)))))(input)?;

    for (_, other_expr) in other_exprs {
        expr = Expr::EBinop(Binop::BAnd, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn or_expr(input: Input) -> IResult<Input, Expr> {
    let (input, (mut expr, other_exprs)) =
        tuple((and_expr, many0(tuple((Token::VerticalBarVerticalBar, and_expr)))))(input)?;

    for (_, other_expr) in other_exprs {
        expr = Expr::EBinop(Binop::BOr, Box::new(expr), Box::new(other_expr))
    }

    Ok((input, expr))
}

fn cond_expr(input: Input) -> IResult<Input, Expr> {
    or_expr(input)
}

fn assign_expr(input: Input) -> IResult<Input, Expr> {
    alt((
        map(
            tuple((unary_expr, Token::Eq, assign_expr)),
            |(assignee, _, assigned)| Expr::EAssign(Box::new(assignee), Box::new(assigned)),
        ),
        cond_expr
    ))(input)
}

data_variant_parser! {
        fn hex_constant(input) -> Result<Const>;
        pattern = Token::HexConstant(x) => x;
    }
data_variant_parser! {
        fn decimal_constant(input) -> Result<Const>;
        pattern = Token::DecimalConstant(x) => x;
    }
data_variant_parser! {
        fn octal_constant(input) -> Result<Const>;
        pattern = Token::OctalConstant(x) => x;
    }

data_variant_parser! {
        fn char_constant(input) -> Result<Const>;
        pattern = Token::CharConstant(x) => x;
    }

fn integer(input: Input) -> IResult<Input, Const> {
    alt((
        hex_constant,
        decimal_constant,
        octal_constant,
        char_constant,
        map(Token::NewlineConstant, |_| 10)
    ))(input)
}

#[cfg(test)]
pub mod tests {
    use std::assert_matches::assert_matches;
    use std::vec;
    use logos_nom_bridge::Tokens;

    use crate::parser::*;
    use crate::parser::{parse_file};

    macro_rules! parse {
            ($name:ident: $fun:ident($content:literal) -> $expr:expr) => {
                #[test]
                fn $name() {
                    let tokens = Tokens::new($content);
                    let parsed = tuple(($fun, eof))(tokens);
                    assert_matches!(parsed, Ok(_));
                    let (_, (parsed,_)) = parsed.unwrap();
                    assert_eq!(parsed, $expr)
                }
            };
        }

    #[test]
    fn test_eof() {
        let to_test = "x";
        let tokens = Input::new(to_test);

        assert_matches!(
            tuple((Token::Ident, eof))(tokens),
            Ok(_)
        )
    }

    parse!(int_decimal: integer("1") -> 1);
    parse!(int_octal: integer("0") -> 0);
    parse!(int_hex_x_0: integer("0x0") -> 0);
    parse!(int_hex_x_1: integer("0X1") -> 1);
    parse!(add: expr("x+1") -> Expr::EBinop(Binop::BAdd, Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));
    parse!(empty_stmt: stmt(";") -> Stmt::SSkip);
    parse!(expr_stmt: stmt("x;") -> Stmt::SExpr(Expr::EVar("x")));
    parse!(return_stmt: stmt("return x;") -> Stmt::SReturn(Expr::EVar("x")));
    parse!(assign_var_expr: expr("x = 1") -> Expr::EAssign(Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));

    parse!(eq_expr: expr("x == 1") -> Expr::EBinop(Binop::BEq, Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));
    parse!(and_expr: expr("x && 1") -> Expr::EBinop(Binop::BAnd, Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));
    parse!(or_expr: expr("x || 1") -> Expr::EBinop(Binop::BOr, Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));
    parse!(not_expr: expr("!x") -> Expr::EUnop(Unop::UNot, Box::new(Expr::EVar("x"))));
    parse!(ge_expr: expr("x >= 1") -> Expr::EBinop(Binop::BGe, Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))));

    parse!(call_1: expr("f(x)") -> Expr::ECall(Box::new(Expr::EVar("f")), vec![Expr::EVar("x")]));
    parse!(call_3: expr("f(x,1,2)") -> Expr::ECall(Box::new(Expr::EVar("f")), vec![Expr::EVar("x"), Expr::EConst(1), Expr::EConst(2)]));

    parse!(double_assign_var_expr: expr("y = x = 1") -> Expr::EAssign(Box::new(Expr::EVar("y")), Box::new(Expr::EAssign(Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1))))));
    parse!(assign_var_stmt: stmt("x = 1;") -> Stmt::SExpr(Expr::EAssign(Box::new(Expr::EVar("x")), Box::new(Expr::EConst(1)))));
    parse!(block_raw: block("{int x; x+y;}") -> Block::new(
            vec![Formal::new("x", Typ::TInt)],
            vec![Stmt::SExpr(Expr::EBinop(Binop::BAdd, Box::new(Expr::EVar("x")), Box::new(Expr::EVar("y"))))])
        );

    parse!(parse_fun: decl_fun("int f(int a) { int x; return a;}") -> Fun::new(
            Formal::new("f", Typ::TInt),
            vec![Formal::new("a", Typ::TInt)],
            Block::new(vec![Formal::new("x", Typ::TInt)], vec![Stmt::SReturn(Expr::EVar("a"))]))
        );

    parse!(parse_fun_multiple_args: decl_fun("int f(int a, int b, int c) { }") -> Fun::new(
            Formal::new("f", Typ::TInt),
            vec![Formal::new("a", Typ::TInt), Formal::new("b", Typ::TInt), Formal::new("c", Typ::TInt)],
            Block::new(vec![], vec![]))
        );

    parse!(parse_struct: decl_struct("struct S { int a; };") -> Struct::new("S", vec![Formal::new("a", Typ::TInt)]));

    parse!(file_single_fun: parse_file("int f() {return x;}") -> File::new(
            vec![
                Fun::new(Formal::new("f", Typ::TInt), vec![], Block::new(vec![], vec![Stmt::SReturn(Expr::EVar("x"))]))
            ],
            vec![])
        );

    parse!(file_fun_and_struct: parse_file("struct S { int a; }; int main() { int x; { struct S *x; x->a = 42; } x = 1; }") ->
        File::new(
        vec![
            Fun::new(Formal::new("main", Typ::TInt), vec![], Block::new(
                vec![Formal::new("x", Typ::TInt)],
                vec![
                    Stmt::SBlock(
                        Block::new(
                            vec![Formal::new("x", Typ::TStruct("S"))],
                            vec![Stmt::SExpr(Expr::EAssign(
                                Box::new(Expr::EArrow(Box::new(Expr::EVar("x")), "a")),
                                Box::new(Expr::EConst(42))))]
                        )
                    ),
                    Stmt::SExpr(
                        Expr::EAssign(
                            Box::new(Expr::EVar("x")),
                            Box::new(Expr::EConst(1))
                        )
                    )
                ]
            ))
        ],
        vec![
            Struct::new("S", vec![Formal::new("a", Typ::TInt)])
        ])
    );
}

#[allow(dead_code)]
pub mod structure {
    use derive_new::new;
    use derive_getters::Getters;

    pub type Ident<'a> = &'a str;
    pub type Const = i64;

    #[derive(new, Debug, PartialEq, Getters)]
    pub struct File<'a> {
        funs: Vec<Fun<'a>>,
        structs: Vec<Struct<'a>>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Typ<'a> {
        TInt,
        TStruct(Ident<'a>),
    }

    #[derive(new, Debug, PartialEq, Getters)]
    pub struct Formal<'a> {
        name: Ident<'a>,
        typ: Typ<'a>,
    }

    #[derive(new, Debug, PartialEq, Getters)]
    pub struct Struct<'a> {
        name: Ident<'a>,
        fields: Vec<Formal<'a>>,
    }

    #[derive(new, Debug, PartialEq, Getters)]
    pub struct Fun<'a> {
        profile: Formal<'a>,
        args: Vec<Formal<'a>>,
        body: Block<'a>,
    }

    #[derive(new, Debug, PartialEq, Getters)]
    pub struct Block<'a> {
        vars: Vec<Formal<'a>>,
        stmts: Vec<Stmt<'a>>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Stmt<'a> {
        SSkip,
        SExpr(Expr<'a>),
        SIf(Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
        SWhile(Expr<'a>, Box<Stmt<'a>>),
        SBlock(Block<'a>),
        SReturn(Expr<'a>),
    }

    #[derive(Debug, PartialEq)]
    pub enum Expr<'a> {
        EConst(Const),
        EVar(Ident<'a>),
        EArrow(Box<Expr<'a>>, Ident<'a>),
        EAssign(Box<Expr<'a>>, Box<Expr<'a>>),
        EUnop(Unop, Box<Expr<'a>>),
        EBinop(Binop, Box<Expr<'a>>, Box<Expr<'a>>),
        ECall(Box<Expr<'a>>, Vec<Expr<'a>>),
        ESizeof(Ident<'a>),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Unop {
        UNot,
        UMinus,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Binop {
        BEq,
        BNeq,
        BLt,
        BGt,
        BGe,
        BLe,
        BAdd,
        BSub,
        BMul,
        BDiv,
        BAnd,
        BOr,
    }
}
